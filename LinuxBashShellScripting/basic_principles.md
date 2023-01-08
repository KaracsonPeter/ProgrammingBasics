# Bash shell scripting
The purpose of this documentation is to introduce how powerful you can cooperate with Linux systems 
by using commands built in the system and also automate tasks with them. 
It can be useful for managing Linux servers in every kind of project.  
(These are my notes from an udemy lecture called "Complete Linux Bash Shell Scripting with Real Life Examples" by Imran Afzal.)

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
### Accessing data from file
There could be a lot of case, when you have to look for error messages in a linux file. 
Instead of scrolling in the file for an eternity, you can just list only the error messages from it:
```bash
grep -i error /home/p/file_name
# or
grep -i warn /home/p/file_name
# if something failed write the message into a file:
grep -i fail /home/p/file_name > /home/p/fail_messages
# list hom many errors are there on Aug 27
grep -i error /home/p/file_name > /home/p/fail_messages | grep "Aug 27" | wc -l 
```

```bash
# print only the first column of date result:
date | awk '{print $1}'
# third col:
uptime | awk '{print $3}'
# list disc space only for root
df -h | grep root
```

'|' called "pipe". 
With this, you can pass the output of a bash command to the input of another bash command.  
`ls | wc -l` Here the out of ls command is counted by wc command.

### Check remote servers connectivity
```bash
# -c1 - means do a statistic, -n 10 - means from 10 pings
ping -n 10 -c1 www.google.com &> /dev/null # output the result to nothing, cause we only need the status
	if [ $? -eq 0 ]
	then
	echo OK
	else
	echo NOT OK
	fi
```

You can also put every IP address into a file, from which you can look up and check its connections to your computer:  
```bash
host_list_file="/home/p/helper_files/host_list"

for ip in $(cat host_list_file)
do
    ping -n 10 -c1 ip &> /dev/null
    if [ $? -eq 0 ]
    then
    echo ${ip} is OK
    else
    echo ${ip} is NOT OK
    fi
done
```
`host_list` file content:  
```txt
192.168.1.1
192.168.1.235
```

### Scheduling scripts & send notifications
You can schedule scripts with `crontab`. 
By writing `crontab -e` you can open a file to which you can write when you want to call scripts.
The format: (minute hour day month year path > script)
```txt
8 * * * * /home/bash_scripts/my_script > result
```
This will write the result of `my_script` into `result` file in home.  
You can also send the result an email. E.g:  
```txt
8 13 * * * /home/bash_scripts/my_script | mail -s "test_mail" your_email@gmail.com
```
(You have to set up `sendmail` first, which can cause headache on an Ubuntu running on cloud. Maybe the easiest way is to use Python to send an email.)

### Delete old files
If you are working as a system administrator, it is mandatory to manage diskspace.  
If the disk is full on which an OS runs, the computer becomes unresponsive!  

Preparation:  
```bash
# Create file by modifying its creation time stamp
touch -d "Thu, 1 March 2018  12:30:00" alma
# Get current path
pwd
# Short explanation: find all files created 3 month ago in a specific directory and print permissions, owner, size, and only the size.
# Detailed: find in path those files which older(+) then 90 days and for these, execute the ls -l command (-l = long format).
# by using {}, you can pass file paths found by 'find' to the 'ls' command. '\;' close the flag 'exec'. 
find /home/ubuntu/bash_scripts -mtime +90 -exec ls -l {} \; -exec ls -s {} \;
```

Scripts:
```bash
# This script will delete files older then 90 days in /home/ubuntu/bash_scripts.
find /home/ubuntu/bash_scripts -mtime +90 -exec rm {} \;
```

```bash
# Create an "old" folder if not exist and move every file to it which is older then 90 days.
dir=$1
age=$2

if [ -d "$dir" ]; then
	if [ -z age ]
	then
		echo Age is not defined. Abort.
	else
		find ${dir} -mtime +${age} -exec mv {} ${dir}/old/ \;
	fi
else
	echo Please give an existing directory to be observed!
fi
```

### Backup directories
In the future you my want to back up "/etc" and "/var" directories which contains all your configurations. It can be useful for troubleshooting.  
**Backup to a .tar file and compress its content:**
```bash
tar cvf /tmp/backup.tar /etc /var
gzip /tmp/backup.tar
```

**Script to back up and transfer the file to another location:**
```bash
tar cvf /tmp/backup.tar /etc /var

gzip /tmp/backup.tar
# find zip made today
find backup.tar.gz -mtime -1 -type f -print &> /dev/null

if [ $? -eq 0 ]
	then
 	echo Backup was created
 	echo
 	echo Archiving Backup to another location
 	scp /tmp/backup.tar.gz root@192.168.1.x:/path
else
 	echo Backup failed
fi
```

Other examples for `for` loop:
```bash
echo This script will give write permission to each file in the given directory.
if [ -z $1 ]; then
        echo NOW GIVE ME THE DIRECTORY YOU...!
        sleep 2
        echo Just kidding... you should have already defined it as an argument of the script.
        sleep 1
        echo Abort.
else
        for i in *
        do
                chmod a+w $i
        done
fi
```

```bash
echo This script sets the extension of those files in the passed directiory which have no extension.
echo
echo Specify the directory for which the modification should be applied!
read path
echo $path
echo Specify the extension for which you want to switch.
read extension_to_switch
echo $extension_to_switch

if [ -z "$extension_to_switch" ]; then
        echo extension_to_switch not defined
fi

# You can refer to the output of a command by putting the command between '`' characters.
for file in `ls $path --ignore='*.*'`; do
        mv $file $file.$extension_to_switch
done
```

```bash
# Description: This script will delete the chosen string from the end of each file exist in the given directory.
path=$1
str=$2

for file_name in `ls $path`; do
        mv ${file_name} ${file_name%$str}
done
```

```bash
# Description: This script check if the hardcoded files are exist.
FILES="/etc/passwd
/etc/group
/etc/shadow
/etc/nsswitch.conf
/etc/sshd_ssh_config
/etc/fake
"
for file in $FILES; do
        if [ ! -e $file ]; then
                echo $file do not exist!
        fi
done
```

### Copy files to remote host
```bash
# Description: This script will copy files to remote hosts.
host_names= `cat /home/name/host_name_list`

for host_name in $host_names; do
        scp some_file $host_name:/server_path
done
```

### User directory assignment
```bash
# Description: Find those directories which has no assigned user.
cd /home

for DIR in *; do
        CHECK=$(grep -c "/home/$DIR" /etc/passwd)
        if [ ! $CHECK -ge 1 ]; then
                echo "$DIR is NOT assigned to a user"
        fi
done
```

### List users who have been logged in on a specific day
```bash
# Description: This script prints user names logged in today.
# Get only the first 3 output of date (day_of_the_week month day)
today=`date | awk '{print $1, $2, $3}'`
# grep only today's data and get only user names by awk
last | grep "$today" | awk '{print $1}'
``` 

```bash
# Description: This script collect users logged in a specific day.
echo "Please enter a month (e.g. Aug)"
read m
echo "Please enter date (e.g. 28)"
read d
echo
last | grep "$m $d" | awk '{print $1}'
```

### Script for central logging (rsyslog)
```bash
# tail reads specific part of the file. By -fn, you will only reed the freshest unread lines from the file.
# 'while read' means that it reads lines until it finds.
tail -fn0 /var/log/messages | while read line
do
        # egrep can take multiple keyword efficiently
        echo $line | egrep -i "refused|invalid|error|fail|lost|down|offline"
        if [ $? = 0 ]; then # if the exit status is 0 --> it found something
                echo $line >> /tmp/filtered-messages # echo that output to a file
        fi
done
```
You can test it's working by logger -t invalid "test invalid".  
[More about tail](https://ss64.com/bash/tail.html)

You can run the script and get your terminal back at the same time by:  
`nohup /root/script_name &`  
You can check if it is running by:  
`ps -ef | grep script_name`  
Here you can see the name of the process by which you can kill the process:  
`kill ID` or `process_name`
```bash
:~ps -ef | grep who_logged_in.bash
user       649    10  0 15:43 pts/0    00:00:00 /bin/sh ./who_logged_in.bash
user       745    10  0 15:45 pts/0    00:00:00 grep --color=auto who_logged_in.bash
:~/bash_scripts$ kill 649
```

```bash
# Description: This script will send an email to administrator

IT="admin_1@hotmail.com, admin_2@gmail.com"

if [ -s /tmp/filtered-messages ]; then # if this file exist
        # unique means combining redundant messages. That is why we need sort. Then mail it
        cat /tmp/filtered-messages | short | unique | mail -s "syslog message" $IT 
        rm /tmp/filtered-messages
else
fi
```

### User account management
Automate user account creation
Create user:  
```bash
# Description: This script will create a user account

echo "Please provide a username!"
read u
echo

useradd $u
echo $u account has been created
```

Check if user exist:  
```bash
echo "Please provide a username!"
read u
echo

grep -q $u /etc/passwd
        if [ $? -eq 0 ]; then
                echo ERROR -- User $u already exists
                Echo Please choose another username
                echo
                exit 0
        fi

useradd $u
echo $u account has been created
```

Force user to add user description:  
```bash
echo "Please provide a username!"
read user_name
echo

grep -q $user_name /etc/passwd
        if [ $? -eq 0 ]; then
                echo ERROR -- User $user_name already exists
                Echo Please choose another username
                echo
                exit 0
        fi

echo "Please provide user description!"
read description
echo

useradd $user_name -c "$description"
echo $user_name account has been created
```

Specify the user ID:  
```bash
echo "Please provide a username!"
read user_name
echo

grep -q $user_name /etc/passwd
        if [ $? -eq 0 ]; then
                echo ERROR -- User $user_name already exists
                Echo Please choose another username
                echo
                exit 0
        fi

echo "Please provide user description!"
read description
echo

echo "Do you want to specify user ID (y/n)?"
read ynu

if [ "$ynu" = "y" ]
then
        echo "Please enter user ID!"
        read uid

        useradd $user_name -c "$description" -u $uid
        echo $user_name has been created
elif [ "$ynu" = "n" ]
then
        echo No worries, we will assigne a user ID
        useradd $user_name -c "$description"
fi
```

Error out if user ID exist:  
```bash
echo "Please provide a username!"
read user_name
echo

grep -q $user_name /etc/passwd
        if [ $? -eq 0 ]; then
                echo ERROR -- User $user_name already exists
                Echo Please choose another username
                echo
                exit 0
        fi

echo "Please provide user description!"
read description
echo

echo "Do you want to specify user ID (y/n)?"
read ynu

if [ "$ynu" = "y" ]; then
        echo "Please enter user ID!"
        read uid
        grep -q $uid /etc/passwd
        if [ $? -eq 0 ]; then
                echo ERROR -- user ID $uid already exist
                exit 0
        else
                useradd $user_name -c "$description" -u $uid
                echo $user_name has been created
        fi
elif [ "$ynu" = "n" ]; then
        echo No worries, we will assigne a user ID
        useradd $user_name -c "$description"
fi
```

### Disable inactive users
```bash
# lastlog is a command to  list all the users had been logged in. tail -n+2 will get rid of the first line of the output.
# lastlog -b 90  will give you all users have not been logged in the last 90 days.
# we will only get test account that have the word 'test' in their names
accounts=`lastlog | tail -n+2 | grep 'test' | awk '{print $1}'`

for acc in $accounts; do
        usermod -L $acc
done
```
To enable account again, you can delete disabling from `/etc/shadow` by deleting '!' behind 'ID:'.  
  
The same with one line:  
```bash
lastlog | tail -n+2 | grep 'test' | awk '{print $1}' | xargs -I{} usermod -L {}
```

### Check process status and killing it
Kill zombie processes:
```bash
# Launch a process which won't stop for a long time:
:~nohup sleep 1000 &
# Find the process:
:~ps -ef | grep "sleep 1000"
user       991    10  0 14:37 pts/0    00:00:00 sudo nohup sleep 1000
user       998    10  0 14:39 pts/0    00:00:00 grep --color=auto sleep 1000
# Filter out grep:
:~ps -ef | grep "sleep 1000" | grep -v grep
# Get only the process ID
:~ps -ef | grep "sleep 1000" | grep -v grep | awk '{print $2}'
990
# Pass IDs to echo command:
:~ps -ef | grep "sleep 1000" | grep -v grep | awk '{print $2}' | xargs -I{} echo {}
990
# Find and kill the process by its ID (pass it to kill instead of echo):
:~ps -ef | grep "sleep 1000" | grep -v grep | awk '{print $2}' | xargs -I{} kill {}
```

### Disk space status
```bash
# Get disk status
:~df -h
Filesystem      Size  Used Avail Use% Mounted on
tmpfs           3.1G     0  3.1G   0% /mnt/wsl
devtmpfs        3.1G     0  3.1G   0% /sys/fs/cgroup
drivers         472G  182G  291G  39% /usr/lib/wsl/drivers
lib             472G  182G  291G  39% /usr/lib/wsl/lib
C:\             472G  182G  291G  39% /mnt/c
# Get rid of 'tmpfs' and 'devtmpfs' | delete the first line | get the 5th column | cut down '%' sign (-f1 means getting the first part of the cut str)
:~df -h | egrep -v "tmpfs|devtmpfs" | tail -n+2 | awk '{print $5}' | cut -d'%' -f1
39
39
39 
```

Script:
```bash
partition_status_list=`df -h | egrep -v "tmpfs|devtmpfs" | tail -n+2 | awk '{print $5}' | cut -d'%' -f1`

for partition_status in $partition_status_list; do
        if [ $partition_status -ge 30 ]; then
                echo Check disk space $partition_status `df -h | grep $partition_status`
        fi
done
```

Another way to list everything above a threshold:  
```bash
:~df -h | awk '0+$5 >= 10 {print}'
# printing only the percentage and the name of the partition
:~df -h | awk '0+$5 >= 10 {print}' | awk '{print $5, $6}'
39%/init
39%/usr/lib/wsl/drivers
39%/usr/lib/wsl/lib
39%/mnt/c
```
