## What is a Kernel?
An interface between your HardWare (HW) and SoftWare (SW).
It is part of the Operating System (OS), and it takes commands from the Shell.
Shell can be used by a Graphical User Interface (GUI) or just a terminal. 
Shell and Kernel together are called the Operating System.
There are many shells. One of them called Born Again Shell (BASH).

## What is a Shell?
It is an Interface between Users and Kernels.
On Windows when you double-click on an image of a program, you are executing a command in whe Windows GUI shell.
On Linux, you can list all the available shells by running the next command from a terminal:  
`cat /etc/shells`  
When you interact with any shell through a GUI, you are actually running shell scripts written by the developers of the OS.
These scripts are executed by the OS sequentially. 
You can also write scripts like this on your own and that is what this course is about.  
Different types of shells (just for fun... in this course we only use bash):  
 * Gnome: Linux GUI shell
 * KDE
 * sh: on of the original Linux shells
 * bash: born again shell
 * csh, tcsh
 * ksh

By typing `echo $0` you can check what type of shell is used by your terminal currently.  
By typing `ls -l` you can list what is in your home directory.  
By typing the name of an available shell, you can use it. E.g.: `sh`  
By typing `exit` you can exit from the chosen shell. (If you type it in the default shell the terminal closes because there won't be any active shell it could interact with.)  

## Script standards
Conventions:
 * Collect all the scripts into one organized folder
 * Name it that matches its purpose
 * Involve the name of the executer shell as its extension e.g.: `do_something.bash`

### File permissions
a+x means every user should have executable permission to this script.  
`chmod a+x script_name`  
List available scripts with their permission:  
`ls -ltr`  

### Script format & structure
 * You have to start your script by some descriptive comments:
   * The shell: #!/bin/bash
   * Date
   * Last modification date
   * What my script does
 * Variable definitions
 * Actual script  

E.g.:
```bash
#!/bin/bash
# Purpose: Testing script format.
# Date: 08/20/2020
# Modifications: 02/08/2022
# This script ensures you an insight how bash script format looks like.

echo "My first string to echo"
```

## Scripting
You can run any command in the script that you can run in bash shell.  
E.g.: `ls` will list the current files and folders available in the current directory.  

This example does basic bash shell operations:  
```bash
# Purpose: Test script
# Date: 08/20/2020
# Modifications: 02/08/2022

echo 
# Launches "task manager" and prints only its first 10 row
top | head -10
echo
ls
echo
echo Write something
# Get user input
read read_string
echo ${read_string} had been read
echo
my_command='hostname'
echo My hostname is $my_command
echo
count=100
if [ $count -eq 100 ]
then
	echo count is 100
else
	echo count is not 100
fi
echo
# Check is file exists. If not, create it.
if [ -e ./bash_scripts/errors.txt ]
then
	echo File exist
else
	echo File does not exist
	echo Creating errors.txt
	touch ./bash_scripts/errors.txt
fi
echo
```

### case
Almost every bash option works based on this. 
```bash
echo 'Enter 'a' to print date and times.'
echo 'Enter 'b' to list all directories and files in this folder'
echo
	read choices
	
	case $choices in
	
a) date;;
b) ls;;
*) echo invalid command
	esac
echo
```

### for loop
```bash
for i in {1..3}
# for i in 1 2 3
do
echo i is $i
done
```

### do-while script
```bash
count=0
num=10
while [ $count -lt 10 ]
do
	echo $num seconds left to stop process $1
	sleep 1
num=`expr $num - 1`
count=`expr $count + 1`
done
```

### exit status
By entering `echo $?` give you the result of the previously ran bash command.
 * 0 = OK
 * 1 = minor problem
 * 2 = serious problem
 * 3-255 = everything else  

## Real life scripts
