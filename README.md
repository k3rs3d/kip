This is a simple command-line tool to assist with IP addressing and subnetting. 

Input an IPv4 address along with either a CIDR notation or a subnet mask, and it calculates various details, like the network address, broadcast address, range of usable IP addresses. 

I wrote this as a networking student to help me prepare for the CCNA certification.

## Usage

**IPv4 Address with CIDR Notation:**
   Provide the IP address followed by the CIDR value as a second argument (not with the slash notation).

   ```sh
   kip 192.168.1.10 24
   ```

**IPv4 Address with Subnet Mask:**
   Provide the IP address followed by the subnet mask.

   ```sh
   kip 192.168.1.10 255.255.255.0
   ```
