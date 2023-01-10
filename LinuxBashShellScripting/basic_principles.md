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
**Note:**  
The syntax between [] in bash scripting is known as a [**test operator**](https://kapeli.com/cheat_sheets/Bash_Test_Operators.docset/Contents/Resources/Documents/index),
and it is used to test for various conditions, such as whether a file or directory exists, or whether a variable is set or not.  
Some flags with which you can **test if**:
 * -d: file is a directory
 * -e: file exists
 * -f: file is a regular file
 * -r: file is readable
 * -w: file is writable
 * -x: file is executable

## Linux basics
### exit status
By entering `echo $?` give you the result of the previously ran bash command.
 * 0 = OK
 * 1 = minor problem
 * 2 = serious problem
 * 3-255 = everything else  

### File types
There are 7 different filet types

| File symbol | Meaning      |
|-------------|--------------|
| -           | Regular file |
| d           | Directory    |
| l           | link         |
| c           | Special file or device file |
| s           | socket       |
| p           | Named pipe   |
| b           | Block device |

### Wild cards
Wild cards are used to get the output what we are looking for.  
The most basic ones:  
 * '*' - represents zero or more characters
 * '?' - represents a single character
 * '[]' - represents a range of characters
 * '{}'

Example:
```bash
# create 8 files
touch my_files{1..8}
# list all these files
ls -l my_files*
# give me anything that has 'm' of 'i' in its name
ls -l *[my]*
# delete everything which has "my" in its name
rm *my* 
```

### Soft links and hard links
**inode (index node)** = A pointer to a file on your disk. It stores data such as the file's size, ownership, 
and access permissions, as well as pointers to the blocks on the disk where the file's data is stored.
When a file or directory is accessed, the operating system uses the inode to locate the file's data and retrieve it for the user.

**soft link (symbolic link)** = A special type of file that **points to another file or directory** (and not for the inode). 
It is similar to a shortcut in Windows. 
It is a type of file that is represented by the file symbol "l" in the output of the "ls -l" command.
When you access a symbolic link, you are actually accessing the file or directory that it points to.
If the original file is deleted or moved, the soft link will become invalid and will not be able to access the target file or directory.

**Hard link** = Deleting, renaming or moving the original file will not affect the hard link, 
because it points directly to the inode of the file. 

Differences:  

| Action                         | Soft link             | Hard link                       |
|--------------------------------|-----------------------|---------------------------------|
|Creatable and interpretable in  | Multiple file systems | Only in in the same file system |
| After deleting the target file | Become invalid        | No affect                       |

```bash
# Creating a soft link
ln -s /path/to/original/file /path/to/link
# Create hard link
ln /path/to/original/file /path/to/link
cat link
# prints the same as
cat file
# to list the long form of files (l), sort them by their creation time (t), 
# revers the order (r) to see the latest the lowest, include the inode  number, write: 
ls -ltri
#-------------------------------------------------------
# Soft link has a different inode as the original file (31138, 31139)
:~/links$ ln -s ../bash_scripts/example_1
:~/links$ ls
example_1
:~/links$ ls -ltri
total 0
31138 lrwxrwxrwx 1 peti peti 25 Jan  6 12:34 example_1 -> ../bash_scripts/example_1
:~/links$ ls -ltri ../bash_scripts/example_1
31119 -rwxrwxrwx 1 peti peti 5 Jan  4 08:39 ../bash_scripts/example_1
# If we delete the original file, the link wont find it
# If we recreate it where it were originally, it will find it again, because the link holds the path to the file. 
# If we move the original file the link wont find it.
:~mv bash_scripts/example_1 tmp/example_1
#-------------------------------------------------------
# Hard link has the same inode as the original file has
bash_scripts:~echo "123" >> example_1
bash_scripts:~cat example_1
123
# If you modify the source file it will modify the destination link
:~/links$ cat example_1
123
# If we remove the original file
:~/bash_scripts rm example_1
# The link still has the content
:~/links$ cat example_1
123
# Data won't get deleted if the original file or even one hard link points to it
```

### File ownership
There can be 2 owners of a file or directory:
 * A user
 * A group

Only the root can change ownership of a file:
```bash
# Change ownership to root
chown root file_name
# Change owner group
chgrp root file_name
# In your own directory in home you have permission to read and write everything regardless of the ownership.
```

### Combining and splitting files
```bash
# Splitting into 20 line files
split -l 20 single_file unique_files
```

### vi
vi is available on most unix environment.
A few key combination:
By 'esc', you can escape from editor mode to enter command mode.
You can go back to insert mode by hitting 'i' or by just simply start typing.
In command mode you can save the file by type and enter `:qw!`. (`:q! ` is just quit)
In command mode you can delete a line by pressing 'd' and using arrows. You can undo it by pressing 'u'.
By pressing 'x' in command mode, you can delete the character on which your cursor is.
By pressing 'r' you can replace a character on the next hit.

### sed command
```bash
# Replace a word in the printed text of a file (not in the file) with another one (g means global)
sed 's/word_to_replace/new_word/g' file_name
# Replace a word in an entire file with another one
sed -i 's/word_to_replace/new_word/g' file_name
# Delete the line in which a specific string is present
sed '/specific_string/d' file_name
# Delete empty lines (any line (^) that ends with nothing ($))
sed -i '/^$/d'
# Delete the 1st line of the file and write the result into another one
sed '1d' file_name >> file2_name
# Replace tabs
sed s'/\t/ /'g file_name
# Get only line 10 to 20 file content
sed -n 10,20p file_name
# Get all lines except line 10 to 20
sed 10,20d file_name
# Change string in every line except in line 8
sed '8!s/string_to_replace/replace_for/' file_name
```

### System utility commands
 * date
 * uptime: gives information how long the system runs
 * hostname: gives your device hostname
 * uname
 * which: gives where a given command is located (in general /usr/bin)
 * cal: gives a calendar (e.g. cal 9 1997)
 * bc: basic calculator
### System logs monitor
Your system's log can be find in: `/var/log`.
It contains:
 * boot: Contains boot logging. If you reboot your system the boot.log will be overwritten.
 * chronyd = NTP
 * cron: Whenever you create and run an automation in crontab, the executions get logged into cron.
 * maillog: Every time an email is sent out or an email comes in, it gets logged into maillog. 
   If you are not able to send emails, you should check that here.
 * secure: Contains every login information. When, who and with what outcome tried to log in.
 * message: Everything is logged into this. You can find errors, warning, etc. here.
 * httpd

### Aliases
```bash
:~/links$ ls
e  e2
# Making an alias
:~/links$ alias ls="ls -al"
:~/links$ ls
total 16
drwxr-xr-x  2 peti peti 4096 Jan  6 14:53 .
drwxr-x--- 10 peti peti 4096 Jan  6 14:53 ..
-rw-r--r--  1 peti peti   12 Jan  6 14:53 e
-rw-r--r--  1 peti peti    8 Jan  6 14:50 e2
# Remove alias
:~/links$ unalias ls
# list existing aliases
alias
```
You might want to set up your own alias list.
You can write a script which sets up a terminal with your aliases.

### NIC bonding (Network Interface Card)
Behind each ethernet port of a good laptop or PC there is a NIC.
NIC bonding is the combination of multiple NIC into a single bond interface.
 * We do it for the redundancy. We bound two port together for the case when one of them dies.
 * Another reason could be to double communication speed of the port.

There will be 3 files in this process. Two for the two ethernet connection and on bond file which bonds them together.
 * Add a new NIC if it does not exist
 * Install nod driver= `modprobe bonding`
 * To list the bonding module info and its version= `modinfo bonding`
 * Create a file called `ifcfg-bond0`:
```text
DEVICE=bond0
TYPE=Bond
NAME=bond0
BONDING_MASTER=yes
BOOTPROTO=none # or static if you wanna assign static IP address
ONBOOT=yes # enable when system reboots
IPADDR=192.168.1.80 # Have to choose an IP which is not used. You can confirm that by ping it.
NETMASK=255.255.255.0
GATEWAY=193.168.1.1
BONDING_OPTS="mode=5 miimon=100"
```
 * Edit the 1st NIC related config file in `/etc/sysconfig/network-scripts`:
```text
TYPE=Ethernet
BOOTPROTO=none
DEVICE=enp0s3 # Comes from `ifconfig | more`
ONBOOT=yes
HWADDR=08:00:27:c3:0e:28 # MAC address of the device
MASTER=bond0
SLAVE=yes
```
 * Create and edit the 2nd NIC related config file in `/etc/sysconfig/network-scripts`:
```text
TYPE=Ethernet
BOOTPROTO=none
DEVICE=enp0s8 # Comes from `ifconfig | more`
ONBOOT=yes
HWADDR=08:00:27:c3:0d:9b # MAC address of the device
MASTER=bond0
SLAVE=yes
```
 * Restart the system: `systemctl restart network`
 * No you can see bond0 with an IP address by running `ipconfig | more`
 * You can check if it is working by connecting to it through SSH with e.g. putty

### File transfer
#### ftp
Currently, we do not have ftp server, so we will try to connect to redhat server for demonstration purposes:
```bash
ftp fpt.redhat.com
# Now you have to log in to redhat
```
If we try to connect to our own OS by ftp, it won't work because currently there is no running ftp daemon on it:
```bash
:~$ ftp OSuserID
ftp: connection: Connection refused
ftp>
```
ftp daemon is runs in the background on a server and allows clients to connect and transfer files using File Transfer Protocol (FTP).

#### scp
```bash
:~$ scp file_name ip_addr:/home/user/rest/of/the/path # ip_addr can be localhost
... password:
file_name                 100%  7   0.5KB/s   00:00
```

#### rsync
It is famous now (2022).

### RAID (Redundant Array of Independent Disks)
If on disk dies, you have another disk. 
 * RAID0: You have multiple disks to increase performance, BUT it is not redundant. If you lose a disk you lost the data.
 * RAID1: You have an extra copy of the data on another disk. If one disk fails, the operation still can continue. It is slow, because you have to replicate data.
 * RAID5: Some kind of magic... calculates parity bits to the data from which it can recover is somehow. At least 3 disk is necessary.
 * RAID6: Same as RAID5 but it uses 2 parity blocks instead of one. Additional redundancy for the case of multiple disk failure.
 * RAID10: Combines RAID1 with RAID0. To improve performance additionally.
 * RAID50: Combines RAID5 with RAID0. To improve performance additionally.
 * RAID60: Combines RAID6 with RAID0. To improve performance additionally.
 
### Securing Linux Machine (OS Hardening)
What makes part of it?
 * User account management
 * Remove unwanted packages
 * Stop un-used services
 * Disable un-used listening ports... stop them
 * Secure SSH configuration
 * Enable firewall (iptables/firewalld)
 * Enable SELinux
 * Change listening services port numbers
 * Keep your system up to date

#### User account management
Pick a hardly guessable username and ID. E.g.: Peter123 & choose an ID above 10000.
```bash
# List users:
:~$ cat /etc/passwd
# Get user account properties:
:~$ chage -l user_name
Last password change                                    : Dec 12, 2022
Password expires                                        : never           # This is not a good signe
Password inactive                                       : never
Account expires                                         : never
Minimum number of days between password change          : 0
Maximum number of days between password change          : 99999
Number of days of warning before password expires       : 7
# You can set your own stander for the properties above.
# To get how, type:
:~$ chage --help
# You can also see these properties at the end of the lines of this:
:~$  cat /etc/shadow
# You should also analyze this file
cd /etc/pam.d
more system-auth
```

#### Remove unwanted packages
```bash
# To get all your available packages:
rpm -qa
# To get rid of a package:
# !!! Be careful... a lot of package is dependent on other packages !!!
rpm -e package_name
```

#### Stop un-used services
```bash
#To list running services type:
systemctl -a # this will give you everything active or inactive
```

#### Stop un-used listening ports
```bash
# List all the ports that are open and listening right now:
netstat -tunlp
# After IP addresses, you can find open ports (after :)
```
Lot of port has its own purpose. E.g.: 53 is for DNS. If your machine is not a DNS server, then why do you listening on port 53?
SSH listening on port 22 by default. This and other properties can be modified in `/etc/ssh/sshd_config` file to make SSH more secure.
E.g.: I will change the (commented out) `PermitRootLogin` to `no`. This means if anybody logging in the system as root, I will deny it.

#### Enable firewall (iptables/firewalld)
You can tell the firewall e.g. that any traffic comes in, only accept it on port 22. It allows the traffic to come in or to leave the system.
On Linux GUI you can type: `firewall-config` to open a GUI where you can:
 * Enable services
 * Enable ports
 * Add ports
 * Remove protocols / add protocols

By enabling firewall on your system, by default it will block everything!!! So first you have to add a port on which you can communicate.  
  
If you do not ha a GUI:  
`firewall-cmd --help`  

In older Linux versions we have:  
`iptables --help`  
You can modify iptables config in:  
`more /etc/sysconfig/iptables-config`  
If you don't want to modify it with the command line, you can modify it straight in this file and then restart the service.

#### Enable SELinux (Security Enhance Linux)
SELinux (Security-Enhanced Linux) is a security module for the Linux operating system that provides mandatory access control (MAC) for all system processes, files, and users. It is designed to prevent unauthorized access or tampering with system resources, and to enforce the security policies defined by the system administrator. SELinux operates by assigning labels to system objects (such as files and processes) and defining rules for how these objects can interact with each other. It uses a combination of user, role, and type labels to enforce these rules, and provides a set of tools for managing and enforcing these policies at runtime. SELinux is often used in high-security environments, such as government and military systems, to provide an additional layer of security beyond traditional user-based access controls.
```bash
# Check it SELinux is enabled:
sestatus
# If you wanna disable it:
cd /etc/sysconfig/
more selinux # Here you can modify the SELINUX var to enforcing/permissive/disable (force policies/ send warning / turned off)
# Reboot system
```

#### Keep your system up to date
Anything that is related to security you have to update it!

### SELinux
SELinux is a Linux kernel security module that provides a mechanism for supporting access control security policies, including mandatory access controls.
(It is a project of the United States National Security Agency (NSA) and the SELinux community.)
A linux server contains files, directories, processes, sockets, memory, etc. All of these are belonging to users or groups.
All of these are controlled through `chmod`. This type of control is called DAC (Discretionary Access Control), where user has control over everything.
SELinux goes around DAC. An attacker cannot access files until a permission is not assigned to a user.
All in all, the user cannot change mode (`chmod`) anything he would like.
First, it is recommended to use SELinux only in permissive mode. After you are comfortable using it, you can use it in enforcing mode.  

**!!! If you want to try it out, do a backup of your system !!!**  

SELinux labels every file in your system. This label has 4 different type of information:
 * user
 * role
 * type
 * level

To list the label of a directory: `ls -lz /part/to/dir`
Here you can see the type of the related directory.

To list specific (e.g. `httpd`) processes running on SELinux, you can type: `ps axZ | grep httpd`

SELinux settings, get booleans: `getsebool -a` / `semanage boolean -l`. You should know all of them if you want to use SELinux.  
Modify these booleans: `setsebool -P boolean_name on`  

Check error messages related to SELinux: `journalctl`  

To change the type in a label: `chcon -t httpd_sys_content_t FILENAME` / `semanage -t httpd_sys_content_t FILENAME`

### Network File System (NFS)
It is about sharing your files with other computers. If you link two files through NFS, you can reach each other's files just like it would be on your own system.
There is a server which share its file system and there is a client which mount its file system. (Mounting is the process of making a file system available to the system and its users.)
1. For that you have to install 2 packages: `yum install nfs-utils libnfsidmap`  
2. Once these packages are installed, enable and start nfs services.
   1. `systemctl enable rpcbind`
   2. `systemctl enable nfs-server`
   3. `systemctl start rpcbind` 
   4. `systemctl start nfs-server` 
   5. `systemctl start rpc-statd`
   6. `systemctl start nfs-idmapd`  
   
   You are telling these services to start at boot time.
3. Create NFS share directory and assign permissions (You do not need this step if you already have a directory to share.)
   1. `mkdir /new_dir_name`
   2. `chmod a+rwx /new_dir_name`
4. Modify `/etc/exports` file to add new shared filesystem
   1. I want my directory to be shared only with the following IP.   
   So write the following line into exports: `/new_dir_name    192.168.12.7(rw,sync,no_root_squash)` (If you write '*' instead to the IP everyone will reach the dir.)
5. Export the NFS file system:  
```bash
:~$ exportfs -rv
exporting: 192.168.12.7:/new_dir_name
```


### Resolve system performance issues
It can be the:
 * Processing
 * Disk writing
 * Networking
 * Hardware

Troubleshooting steps:
 1. Check if you are on the right system
 2. Check the disk space if it is full
 3. Check the processing (`top`, `free`, `lsmem`, `/proc/meminfo`, `vmstat`, `pmap <PID>`, `dmidecode`, `lscpu`, `/proc/cpuinf`)
 4. Check disk issues, status (`iostat -y 5`, `lsof`)
 5. Check networking (`tcpdump -i enps03`, `lsof -i -P -n | grep -i listen`, `netstat -plnt`, `ss -plnt`, `iftop`)
 6. Check system uptime (`uptime`)
 7. Check system logs
 8. Check hardware status by logging into system console
 9. Other tools (`htop`, `iotop`, `iptraf`, `psacct`)

"So, these are the few command that you need to know for being a good system administrator." 

```bash
hostname
# disk space
df -h | grep -v tmpfs
# Check what consumes lot of memory
du -ah / | sort -nr | more
# Check processes
top
# How much memory you have, you use / share
free
# List memory tells indo about the memory block size
lsmem
# List CPU properties
lscpu
# A bunch of information of your memory
cat /proc/meminfo
# Get specific process information
pmap <process ID>
# Get HW information
dmidecode | more

# First, find out what interface you want to run your `tcpbump` on.
ifconfig
tcpdump -i enp03
tcpdump -i enp03 | grep 162.243.10.151 # get specific communication for an IP address
# Maybe you should check your firewall if you have communication issues
.
.
.
etc.
```

You should definitely try out every command mentioned above. Interpret what is printed by them.  
You can run `man command_name` also.

### What virtualization is?
What is virtual? Something what physically not existing.  
Virtualization is the process of creating a SW-based or virtual representation of st. such as virtual applications, servers, storages and networks.  
You can install an OS which has a hypervisor layer on top of it. Once you have this hypervisor layer on top of it then you are able to run multiple OSs at the same time.  
Who provide virtualization technologies?
 * WMWare
 * Microsoft Hyper_V
 * Citrix
 * Redhat
 * Oracle
 * Amazon
 * Google
 * IBM
 * Huawei

Some base definitions:
 * Hypervisor = Host / Virtual server:  
Every time you are running your virtual environment on the top of a server, 
the underlying host that is allowing you to run the virtual machine is called hypervisor.  
 * Virtual Machine (VM) = Guest OS: Any OS that is running on the top of a hypervisor layer.
 * Virtualization manager = vCenter, OVM, Manager, etc.: Manages VMs.
 * Virtual desktop: Instead of getting a new laptop for a new project, it is easier to get a virtual desktop on a virtual server.
 * P2V = Physical to virtual: Tool to convert a physical machine to a virtual one.
 * V2V = virtual to virtual
 * V2P is considered only if the virtual machine is not up-scalable enough, and you need better performance.
 * Snapshot: You can save system states, that you can set back if something goes wrong.
 * Clone or cloning: If you need a VM which is the same as the one you have. The copy is called clone.

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
