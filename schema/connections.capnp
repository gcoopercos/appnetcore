@0xad7835400145b2e7;

struct AppPacket {
  packetType: union {
     connectionRequest @0 :ConnectionRequest;
     connectionResponse @1 :ConnectionResponse;
     textMessage @2 :TextMessage;
  }
}

struct ConnectionRequest {
    clientReadHost @0 :Text;
    clientReadPort @1 :Text;
    clientName @2 :Text;
    clientPass @3 :Text;
}

struct ConnectionResponse {
    assignedId @0 :UInt32;
}

struct TextMessage {
    receiverId @0 :UInt32;
    message @1 :Text;
}


