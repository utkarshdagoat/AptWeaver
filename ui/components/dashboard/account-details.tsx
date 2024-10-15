"use client";

import React, { useState } from "react";
import { Label } from "@radix-ui/react-dropdown-menu";
import { Input } from "../ui/input";
import { Button } from "../ui/button";
import { Plugs } from "@phosphor-icons/react";
import Portfolio from "./portfolio";

const AccountDetails = () => {
  return (
    <div className="lg:pl-2 space-y-6">
      <WalletConnect />
      <Portfolio />
    </div>
  );
};

const WalletConnect = () => {
  const [isConnected, setIsConnected] = useState(false);
  const [walletConnectURI, setWalletConnectURI] = useState("");

  const handleConnect = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    console.log("Connecting via WalletConnect with URI:", walletConnectURI);
    setIsConnected(true);
  }

  const disconnectService = () => {
    console.log("Disconnecting WalletConnect service");
    setWalletConnectURI("");
    setIsConnected(false);
  }
  return (
    <div className="bg-gradient-to-bl from-accent/40 via-card to-muted/40 to-[100%] border rounded-lg p-4">
      {!isConnected ? (
        <form onSubmit={handleConnect} className="space-y-1">
          <Label className="text-foreground font-medium">
            Connect via WalletConnect
          </Label>
          <div className="flex flex-row gap-2">
            <Input
              placeholder="wc:"
              className="flex-1"
              onChange={(e) => setWalletConnectURI(e.target.value)}
            />
            <Button disabled={!walletConnectURI} size={"icon"}>
              <Plugs />
            </Button>
          </div>
        </form>
      ) : (
        <div className="space-y-1">
          <Label className="text-foreground font-medium">
            Connected via WalletConnect
          </Label>
          <Button size={"sm"} className="w-full" onClick={disconnectService}>
            Disconnect
          </Button>
        </div>
      )}
    </div>
  );
};

export default AccountDetails;
