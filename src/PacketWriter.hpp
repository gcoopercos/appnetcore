#ifndef _PACKETWRITER_HPP_INCLUDED
#define _PACKETWRITER_HPP_INCLUDED

#include <memory>
#include <string>

#include "Command.hpp"

using namespace std;

class PacketWriter {
  public:
    PacketWriter() {}

    /**
     * Writes a packet for a command out
     * @param command Command to send out
     *
     * @return Number of bytes written.
     */
    static int writeCommandPacket(shared_ptr<Command>, string& host, int port);
};

#endif
