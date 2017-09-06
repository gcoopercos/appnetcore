@0xad7835400145b2e7;

struct AppPacket {
  packetType: union {
     connectionRequest @0 :ConnectionRequest;
     helloMessage @1: HelloMessage;
  }
}

struct ConnectionRequest {
    clientReadHost @0 :Text;
    clientReadPort @1 :Text;
    clientName @2 :Text;
    clientPass @3 :Text;
}

struct HelloMessage {
    message @0 :Text;
}


