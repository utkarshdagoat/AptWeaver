import { Skeleton } from "@/components/ui/skeleton";
import { TokenBoxVariant } from "@/lib/types";
import { useSwapStore } from "@/stores/swap-store";
import { useState } from "react";
import TokenSelector from "./token-selector";

const TokenInput = ({ type }: TokenBoxVariant) => {
  const {
    fromAmountUSD,
    toAmountAPT,
    setFromAmountUSD,
    setToAmountAPT,
  } = useSwapStore();

  const [error, setError] = useState<boolean>(false);
  const [errorMessage, setErrorMessage] = useState<string>("");
  const [isLoading, setIsLoading] = useState<boolean>(false);

  return (
    <div className="flex gap-1 items-center">
      {type === "from" ? (
        <AmountInput
          placeholder={fromAmountUSD ? "0.00" : "--"}
          type="number"
          onChange={(e) => setFromAmountUSD(Number(e.target.value))}
          value={fromAmountUSD}
          error={error}
          errorMessage={errorMessage}
        />
      ) : isLoading ? (
        <SwapAmountSkeleton />
      ) : (
        <AmountInput
          placeholder={"--"}
          disabled={!fromAmountUSD}
          value={toAmountAPT === 0 ? "--" : Number(toAmountAPT).toFixed(4)}
          readOnly
        />
      )}
      {/* <TokenSelector type={type} /> */}
    </div>
  );
};

interface AmountInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  error?: boolean;
  errorMessage?: string;
}

function AmountInput({
  error = false,
  errorMessage,
  ...props
}: AmountInputProps) {
  return (
    <div className="pb-2">
      <input
        className={`bg-transparent font-medium w-full text-4xl py-2 outline-none transition-all duration-300 disabled:cursor-not-allowed disabled:text-muted-foreground disabled:brightness-50 ${
          error && "border-b-2 border-b-destructive text-destructive"
        }`}
        pattern="^-?[0-9]\d*\.?\d*$"
        {...props}
      />
      {error && errorMessage && (
        <span className="text-destructive text-xs mt-1">{errorMessage}</span>
      )}
    </div>
  );
}

function SwapAmountSkeleton() {
  return <Skeleton className="h-10 my-4 w-[250px]" />;
}

export default TokenInput;
