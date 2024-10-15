"use client";

import Link from "next/link";
import { Sheet, SheetTrigger, SheetContent } from "@/components/ui/sheet";
import { Button } from "@/components/ui/button";
import { MenuIcon } from "lucide-react";
import { WaveSawtooth } from "@phosphor-icons/react";
import { useState } from "react";

export function NavBar() {
  const navLinks = [
    { href: "/", label: "Dashboard" },
    { href: "/exchange", label: "Exchange" },
  ];

  const [activeIndex, setActiveIndex] = useState(0);

  return (
    <div className="flex items-center py-4 bg-background justify-between md:justify-normal">
      <Link href="#" className="flex items-center gap-2" prefetch={false}>
        <WaveSawtooth weight="regular" className="h-6 w-6 text-primary" />
        <span className="text-xl font-medium bg-clip-text text-transparent bg-gradient-to-b from-primary-foreground to-muted-foreground tracking-tight">
          AptWeave
        </span>
      </Link>
      <div className="hidden md:flex ml-4">
        {navLinks.map((link, index) => (
          <Link key={link.label} href={link.href} prefetch={false}>
            <Button
              size={"sm"}
              className="text-accent-foreground"
              variant={activeIndex === index ? "linkActive" : "linkHover2"}
              onClick={() => setActiveIndex(index)}
            >
              {link.label}
            </Button>
          </Link>
        ))}
      </div>
      <Sheet>
        <SheetTrigger asChild>
          <Button variant="ghost" size="icon" className="md:hidden float-end">
            <MenuIcon className="h-6 w-6 text-foreground" />
            <span className="sr-only">Toggle navigation menu</span>
          </Button>
        </SheetTrigger>
        <SheetContent side="left">
          <div className="grid w-[200px] p-4 bg-card">
            {navLinks.map((link) => (
              <Link
                key={link.label}
                href={link.href}
                className="text-lg font-medium text-foreground hover:underline underline-offset-4"
                prefetch={false}
              >
                {link.label}
              </Link>
            ))}
          </div>
        </SheetContent>
      </Sheet>
    </div>
  );
}
