Download https://www.balena.io/etcher/ for burning operating system on sd card or hdd or sdd or you can use pi imager file 
go to -> options -> and deselect validate write on success: this will increase speed
eject and reinsert after flashing
1.Now go to drive and make file ssh not ssh text file.
2. wpa_supplicant.conf file for wifi connection
country=US
ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
update_config=1

network={
ssid="WIFI_SSID"
scan_ssid=1
psk="WIFI_PASSWORD"
key_mgmt=WPA-PSK
}
3. enable usb boot go to config.txt file and add in the end of file 
program_usb_boot_mode=1
4. now find ip of rasberry pi or assign static ip 
5. access through putty or in cmd ssh ip of pi
6. The default username is pi and password is raspberry.
7. sudo apt-get update
8. sudo apt-get upgrade
For rdp access
9. sudo apt-get install xrdp
10. install vnc viewer or putty in windows. putty is better for resolution 
11. enter username and password and we are done
12. although copy paste work in remote desktop of putty but winscp tool is good without rdp to transfer files
Download https://www.raspberrypi.com/software/operating-systems/ official operating system of Rasberry