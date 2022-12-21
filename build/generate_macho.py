#!/usr/bin/env python

import os
import sys
import shutil
import string
import datetime
import json


# Make a temporary file of BSD syscalls without any additional shit around
def make_temp_bsd():
	print ("[+] Making temporary system call list in " + PATH_BSD_TEMP_FILE)
	fdin = open(PATH_BSD_SYSCALLS, "r+b")
	fdout = open(PATH_BSD_TEMP_FILE, "w+b")

	data = fdin.read()
	data = data[data.find("0\t"):]  # hacky way to set data ptr to proper field
									# since first syscall is prepended with '0\t'
	fdout.write(data)

	fdout.close()
	fdin.close()
	print ("\tdone")

# Generating BSD system call list as a data structure
# [number, return type, name, number of args, arg...n, source]
def bsd_list_generate():
	print ("[+] Generating system call matrix data structure")
	fdin = open(PATH_BSD_TEMP_FILE, "r+b")
	data = fdin.read()
	fdin.close()

	# Split data by lines
	data = string.split(data, '\n')

	# Remove all empty lines
	for i, line in enumerate(data):
		if not line:
			data.pop(i)

	# Remove all asm comments ';'
	tmp = []
	for i, line in enumerate(data):
		if i == 0 or not line.startswith(';'):
			tmp.append(line)
	data = tmp

	# Crafting syscall table (as 2 dimensional matrix)
	syscall_matrix = []
	for line in data:
		entry = []

		if line.startswith("#"):
			# Apple shitheads are lousy.
			if line.find("#endif") != -1:
				entry.append("#endif")
				syscall_matrix.append(entry)
				continue

			entry.append(line)
			syscall_matrix.append(entry)
			continue

		line = line.replace(" (", "(")
		elems = line.split()

		# Syscall ID
		entry.append(elems[0])

		# Syscall return type
		entry.append(elems[4])

		# Syscall name
		entry.append(elems[5][0:elems[5].find('(')])

		# Enumerating arguments of the syscall
		# This shit is kinda tricky. For couple of reasons: 1) There _are_
		# corner cases (e.g. no args), 2) Unknown # of args per syscall, 3)
		# argument can be more than a tuple e.g. 'struct name arg'
		#
		i = 5
		argnum = 0 # we will use it to count number of args
		arg = "" # we're using it to create each arg
		tmp = [] # temporary list of args for each syscall
		while True:
			# Corner case where syscall takes 0 args (denoted "void")
			if(elems[6] == '}' or elems[6] == "NO_SYSCALL_STUB;"):
				entry.append(str(argnum))
				break
			# If argnum > 0 then it's our first iteration always
			elif(i == 5):
				arg = elems[i][elems[i].find('(')+1:]
				i += 1
				continue
			elif(elems[i] != '}' and elems[i] != "NO_SYSCALL_STUB;"):
				if(elems[i].find(')') != -1):
					arg += ' '+elems[i][:elems[i].find(')')]
					tmp.append(arg)
					argnum += 1
					entry.append(str(argnum))
					break
				if(elems[i].find(',') == -1):
					arg += ' '+elems[i]
					i += 1
					continue
				else:
					arg += ' '+elems[i][:elems[i].find(',')]
					tmp.append(arg)
					argnum += 1
					arg = ""
					i += 1

			else:
				break

		# We're adding args from our temporary list
		for el in tmp:
			entry.append(el)

		# Strip prepended spaces from syscall's args
		for i, elem in enumerate(entry):
			entry[i] = elem.strip()

		syscall_matrix.append(entry)

	'''
	for entry in syscall_matrix:
		print entry
	sys.exit()
	'''

	# Clean-up
	cmd = "rm " + PATH_BSD_TEMP_FILE
	os.system(cmd)

	return syscall_matrix

# This comes in handy when generating HTML output
def determine_highest_num_args():

	highest_num = 0
	for el in bsd_syscall_list:
		if el[0].startswith('#'):
			continue
		if(el[3] > highest_num):
			highest_num = el[3]

	print ("[+] Highest # of args is " + highest_num)

	return highest_num

def make_syscall_file_xrefs():
	print ("[+] Generating tags file for " + PATH_XNU_SOURCE)
	# Stage 0: Generate tags for XNU source
	cmd = PATH_EXUBERANT_CTAGS + " -R " + PATH_XNU_SOURCE
	os.system(cmd)

	# Stage 1: Mine for impl files!
	for i, syscall in enumerate(bsd_syscall_list):
		if syscall[0].startswith('#'):
			continue

		# Technically, we could skip them. They ain't do notin'. So we do.
		# However, since there are quite static throughout releases we can still
		# add them into final output when generating HTML
		#elif syscall[2] == "nosys":
		#    continue
		#elif syscall[2] == "enosys":
		#    continue

		else:
			fd = open("tags", "r+b")
			for line in fd.readlines():
				line = line.strip()
				#if line.find("struct "+syscall[2]+"_args") != -1:
				cols = line.split()
				if syscall[2] == cols[0]:
					if line.find(".h") != -1:
						continue

					# LOL, it's pointless since cols are already splitted
					# hence we can print cols[1] and have the same result
					# look at solution for MACH traps ;)
					data = string.split(line, '\t')
					#print syscall[2] + ": " + data[0] + " " + data[1]
					bsd_syscall_list[i].append(data[1].replace(PATH_XNU_SOURCE, URL_XNU_SOURCE))

	'''
	for elem in bsd_syscall_list:
		print elem
	'''

def generate_json():
	print ("[+] Generating JSON file...")

	with open(OUTPUT_JSON, 'w') as fd:
		json.dump(bsd_syscall_list, fd, indent=4)

	print ("\tdone")

def main():
	# Kung-foo for BSD syscalls
	# bsd_syscall_list contains not only syscalls but also #defines!
	make_temp_bsd()
	global bsd_syscall_list
	bsd_syscall_list = bsd_list_generate()

	# Find what is the highest number of arguments
	# Lawl, but it's actually _pretty_ handy!
	global highest_num_args
	highest_num_args = determine_highest_num_args();
	highest_num_args = int(highest_num_args)

	# Kung-foo for implementation files xrefs
	make_syscall_file_xrefs()

	# Dump bsd_syscall_list as JSON
	generate_json()

	print ("[+] Great success!")

if __name__ == "__main__":

	PATH_XNU_SOURCE = sys.argv[1]
	URL_XNU_SOURCE = "https://opensource.apple.com/source/xnu/xnu-4570.41.2/"
	PATH_EXUBERANT_CTAGS = "/usr/local/Cellar/ctags/5.8_1/bin/ctags"
	PATH_BSD_SYSCALLS = PATH_XNU_SOURCE + "bsd/kern/syscalls.master"
	PATH_BSD_TEMP_FILE = "/tmp/bsd-syscall-tmp"
	OUTPUT_JSON = "osx-bsd-syscalls.json"
	OUTPUT_HTML = "osx-bsd-syscalls.html"

	# mention: timestamp, original list from XNU sources, name/handle/email
	BANNER = "<h1>macOS BSD System Calls</h1>\n"
	v1 = PATH_XNU_SOURCE[PATH_XNU_SOURCE.find("xnu-"):]
	ts = datetime.datetime.now().strftime("%A, %d %B %Y")
	twitter = "<a href=\"https://twitter.com/andrzejdyjak\">@andrzejdyjak</a>"
	BANNER += "<p>Generated from <i><a href=\"" + URL_XNU_SOURCE + "\">" + v1.upper() + "</a></i> on <i>" + ts + "</i> by " + twitter + ".</p>\n"
	BANNER += "<p>Description for <a href=\"osx-bsd-syscalls.json\">JSON dump</a> elements (apart from <i>conditionals</i>):</p>"
	BANNER += "<pre>[\n\tsyscall number,\n\treturn type,\n\tsyscall name,\n\tnumber of args,\n\targ 1, ..., arg n,\n\tsource\n]</pre>"
	BANNER += "<p>You can find github repository <a href=\"https://github.com/dyjakan/osx-syscalls-list\">here</a>. Feedback, ideas, bugs, <i>et cetera</i> &#8211; <a href=\"https://sigsegv.pl\">give me a shout</a>.</p>\n"

	main()