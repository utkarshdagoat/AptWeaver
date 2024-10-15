"use client";

import React, { useEffect, useState } from "react";
import { Button } from "../ui/button";
import { Plus } from "@phosphor-icons/react";
import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useDashboardStore } from "./dashboard-store";

const Accounts = () => {
  const { addresses, activeAddressIndex, setActiveAddressIndex } =
    useDashboardStore();
  return (
    <div className="flex flex-row gap-4 pb-2 lg:flex-col lg:w-[200px] lg:border-b-0 lg:pr-2 lg:pb-0">
      <NewWallet />
      <div className="flex-1 flex flex-row gap-4  lg:gap-2 lg:flex-col overflow-auto no-scrollbar">
        {addresses.map((address, index) => (
          <Button
            key={address}
            size={"sm"}
            className="min-w-36 lg:min-w-0 lg:w-full rounded-full"
            onClick={() => setActiveAddressIndex(index)}
            variant={index === activeAddressIndex ? "shine" : "secondary"}
          >
            {address}
          </Button>
        ))}
      </div>
    </div>
  );
};

const NewWallet = () => {
  const [windowWidth, setWindowWidth] = useState(window.innerWidth);
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    const handleResize = () => setWindowWidth(window.innerWidth);
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  const [derivationPath, setDerivationPath] = useState("");
  const { addAddress } = useDashboardStore();

  const createNewWallet = (e) => {
    e.preventDefault();
    console.log("Creating new wallet with derivation path:", derivationPath);
    addAddress("0x1234567890");
    setIsOpen(false);
  };

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button
          variant={"outline"}
          className="rounded-full border-b"
          size={windowWidth < 1024 ? "icon" : "default"}
          onClick={() => setIsOpen(true)} // Open the modal
        >
          <Plus className="lg:mr-2" />
          {windowWidth > 1024 && "New Wallet"}
        </Button>
      </DialogTrigger>
      <DialogContent className="bg-card">
        <form onSubmit={createNewWallet} className="w-full space-y-4">
          <DialogTitle className="mb-2">New Wallet</DialogTitle>
          <div>
            <Label htmlFor="derivationPath" className="text-muted-foreground">
              Derivation Path
            </Label>
            <Input
              id="derivationPath"
              placeholder="Enter derivation path"
              onChange={(e) => setDerivationPath(e.target.value)}
            />
          </div>
          <Button
            variant={"expandIcon"}
            Icon={Plus}
            className="w-full"
            iconPlacement="left"
            disabled={!derivationPath}
            type="submit"
          >
            Create Wallet
          </Button>
        </form>
      </DialogContent>
    </Dialog>
  );
};

export default Accounts;
