#include "packets.hpp"
#include "PacketReader.hpp"

#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/ip.h>
#include <sys/types.h>
#include <unistd.h>
#include <errno.h>
#include <iostream>
#include <cstdio>



using namespace std;
//using namespace google::protobuf::io;


const int BUF_SIZE=1024;

void PacketReader::run(int portnum) {
    int sockfd;
    uint8_t buf[BUF_SIZE];

    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd < 0) {
        cerr << "Problem opening socket" << endl;
    }

    /* setsockopt: Handy debugging trick that lets 
     * us rerun the server immediately after we kill it; 
     * otherwise we have to wait about 20 secs. 
     * Eliminates "ERROR on binding: Address already in use" error. 
     */
    //optval = 1;
    //setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, 
    //       (const void *)&optval , sizeof(int));

    struct sockaddr_in server;
    int length = sizeof(server);
    bzero(&server, length);
    server.sin_family=AF_INET;
    server.sin_addr.s_addr=INADDR_ANY;
    server.sin_port=htons(portnum);
    if (bind(sockfd, (struct sockaddr*)&server, length) < 0)
    {
        cerr << "Problem binding to receiving socket." << endl;
        perror("binding");
        exit(-1);
    }
    socklen_t from_len;
    struct sockaddr_in from_addr;
    from_len = sizeof(struct sockaddr_in);

    // Get the mapping of command type -> command, which contains the 
    // packet (message) to deserialize
    shared_ptr<map<uint32_t, function<shared_ptr<Command>()>>> cmdCreators = 
        anpackets::getCommandCreators();

    while (1) {
        // Read our udp socket data, the whole packet.
        int numRead = recvfrom(sockfd, buf, BUF_SIZE, 0, 
            (struct sockaddr*)&from_addr,&from_len);

        cerr << "Packet read. Size read: " << numRead << endl;

        if (numRead < 4) {
            perror("recvfrom");
        } else if (numRead > 0) {
            // Grab the command type from the 1st 4 bytes (the 'network long')
            uint32_t lvread = *((uint32_t *)buf);
            uint32_t command_type = ntohl(lvread);
            
            // Already read type long
            readCommandPacket(command_type, &buf[4], BUF_SIZE-4, cmdCreators);
        }
    }
}


/**
 * Reads the size of the packet to come.
 */
void PacketReader::readCommandPacket(uint32_t command_type,
      uint8_t *buf, int bufsize, 
      shared_ptr<map<uint32_t, function<shared_ptr<Command>()>>> cmdCreators) {
   
    shared_ptr<Command> cmd;
    if (cmdCreators->find(command_type) == cmdCreators->end()) {
        cerr << "Packet handler not found for cmd type: " << command_type<< endl;
    } else {
      cmd = (*cmdCreators)[command_type]();

      if (cmd) {
        cmd->writeReceivedData(buf, bufsize);
      } else {
        cerr << "Command not create but cmd creation lambda does exist.  cmd type: " << 
            command_type << endl;
      } 

      cerr << "Command packet read. Returning cmd." << endl;

      // Put the command in the queue
      destination_queue->enqueue(cmd); 
    }
}


