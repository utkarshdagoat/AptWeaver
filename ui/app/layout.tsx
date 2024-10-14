import type { Metadata } from "next";
import localFont from "next/font/local";
import "./globals.css";
import { ThemeProvider } from "@/components/theme-provider";

const geistSans = localFont({
  src: "./fonts/GeistVF.woff",
  variable: "--font-geist-sans",
  weight: "100 900",
});
const geistMono = localFont({
  src: "./fonts/GeistMonoVF.woff",
  variable: "--font-geist-mono",
  weight: "100 900",
});

const neueMontreal = localFont({
  src: [
    {
      path: "./fonts/neue-montreal/PPNeueMontreal-Bold.otf",
      weight: "700",
      style: "normal",
    },
    {
      path: "./fonts/neue-montreal/PPNeueMontreal-Medium.otf",
      weight: "500",
      style: "normal",
    },
    {
      path: "./fonts/neue-montreal/PPNeueMontreal-Book.otf",
      weight: "400",
      style: "normal",
    },
    {
      path: "./fonts/neue-montreal/PPNeueMontreal-Thin.otf",
      weight: "200",
      style: "normal",
    },
  ],
});

export const metadata: Metadata = {
  title: "AptWeaver",
  description: "Controlling Ethereum and bitcoin wallets only by your aptos accounts truly keyless",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} ${neueMontreal.className} antialiased`}
      >
        <ThemeProvider
          attribute="class"
          defaultTheme="dark"
          enableSystem
          disableTransitionOnChange
        >
          {children}
        </ThemeProvider>
      </body>
    </html>
  );
}
