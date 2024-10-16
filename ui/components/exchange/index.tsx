import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { useSwapStore } from "@/stores/swap-store";

import Settings from "./settings";
import TransactionHistory from "./transaction-history";
import { FromAddress, SwapIcon, ToAddress } from "./swap-user-data";
import TokenBox from "./token-box";
import { ArrowUpDown, CheckCheckIcon } from "lucide-react";
import { Button } from "../ui/button";
import ConnectWallet from "./connect-wallet";
import { useToast } from "@/hooks/use-toast";
import { useState } from "react";
import { CopyAddress } from "../dashboard/portfolio/commons";

const Swap = () => {
  const {
    fromChain,
    fromAmount,
    fromToken,
    toChain,
    toToken,
    walletConnected,
    activeAddress,
    setActiveAddress,
    swapEnabled,
    setSwapEnabled,
    isLoading,
    setIsLoading,
  } = useSwapStore();

  const { toast } = useToast();

  const [txHash, setTxHash] = useState<string | null>(null);

  const handleSwapClicked = async () => {
    // TODO: Write handle swap logic
  };

  return (
    <div className="*:w-[480px] mx-auto relative overflow-hidden">
      <Card
        className={`z-50 bg transition-all bg-gradient-to-bl from-accent/40 from-[-20%] via-card to-muted/40 duration-500`}
      >
        <CardHeader className="flex flex-row items-center mb-2 justify-between">
          <CardTitle className="font-bold">Swap</CardTitle>
          <div className="space-x-2">
            <Settings />
            <TransactionHistory />
          </div>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <FromAddress />
          <TokenBox type="from" />
          <SwapIcon />
          <ToAddress />
          <TokenBox type="to" />
        </CardContent>
        <CardFooter className="flex flex-col gap-4">
          {!walletConnected ? (
            <ConnectWallet />
          ) : (
            <Button
              className="w-full text-base"
              size={"lg"}
              variant={"expandIcon"}
              iconPlacement="right"
              Icon={ArrowUpDown}
              disabled={!swapEnabled}
              onClick={handleSwapClicked}
            >
              Swap
            </Button>
          )}
          {txHash && (
            <Alert className="">
              <AlertTitle className="text-primary brightness-125 font-bold inline-flex gap-1 items-center">
                Transaction Successful <CheckCheckIcon className="w-4 h-4" />
              </AlertTitle>
              <AlertDescription className="text-muted-foreground font-semibold inline-flex items-center gap-2">
                Transaction Hash:{" "}
                <CopyAddress className="text-sm" address={txHash} description="Transaction Hash copied to clipboard" />
              </AlertDescription>
            </Alert>
          )}
        </CardFooter>
      </Card>
    </div>
  );
};

export default Swap;
