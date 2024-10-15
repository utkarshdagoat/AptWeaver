import PortfolioOverview from "./portfolio-overview";
import { AssetChart } from "./asset-chart";

const Overview = () => {
  return (
    <div className="flex flex-col lg:flex-row gap-2 lg:h-[18rem]">
      <PortfolioOverview />
      <AssetChart />
    </div>
  );
};

export default Overview;
