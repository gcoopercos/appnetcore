#include "PacketWriter.hpp"

#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <iostream>
#include <flatbuffers/flatbuffers.h>

using namespace std;


int PacketWriter::writeCommandPacket(
    shared_ptr<Command> command,
    string& host,
    int port) {

    flatbuffers::FlatBufferBuilder& fbb = command->getBufferBuilder();
    int size = fbb.GetSize() + 4; // Need room for the type
    if (size > 1000) {
        cerr << " Warning: packet size too large" << endl;
    }
    
    uint8_t* pkt_notype = fbb.GetBufferPointer();
    uint8_t pkt[1024];
    uint32_t commandType = htonl(command->getCommandTypeId());

    memcpy(pkt, &commandType, 4);
    memcpy(pkt+4, pkt_notype, fbb.GetSize());


    int sock_fd;
    sock_fd = socket(AF_INET, SOCK_DGRAM,0);
    struct sockaddr_in server; //, from;
    server.sin_family = AF_INET;
    //server.sin_7port = htons(port);
    server.sin_port = htons(port);
    memset(&(server.sin_zero), 0, 8);
    server.sin_addr.s_addr = inet_addr(host.c_str());


    if (sock_fd < 0) {
        cerr << "socket(..) failed" << endl;
    }

    int n=sendto(sock_fd, pkt,size, 0, (const struct sockaddr *)&server, sizeof(server));

    return n;
}



