#!/bin/bash

################################################################################
# Syntactic breakdown of the `find` command.
# - Could be of use to anyone who wants to do shell automation...
# https://www.howtogeek.com/771399/how-to-use-the-find-command-in-linux/
################################################################################
# Mandatory arguments to use the find command.
################################################################################
# . -> current dir.
#
# -name "name_pattern" -> In this example is *.jpg aka all files which end in
# .jpg (the asterisk symbol - * - is a wildcard for any pattern).
#
################################################################################
# Optional arguments for this context. (although useful overall)
################################################################################
# -type f -> Only show files. (Optional)
#
# -exec -> After this flag you enter the command you want to execute (for
# example: wc -c "{}" -> Which counts the lines for a file). Chaining find with
# commands is a very common pattern and a very useful way to do automated
# tasks although not needed in this case. (Optional)
#
# \; -> If you choose to use a command you need to escape it with a \ symbol
# and end the execution with a semicolon (Otherwise the \ symbol will only pass
# you to the next line where the shell expects more things to get added to the
# command). (Optional)
################################################################################
find . -name "*.jpg" -type f -exec wc -c "{}" \;
