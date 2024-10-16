import React from "react";
import Accounts from "./accounts";
import AccountDetails from "./account-details";

const Dashboard = () => {
  return (
    <div className="w-full">
      <h1 className="text-2xl">Dashboard</h1>
      <div className="mt-4 h-[90vh] lg:h-[80vh] flex flex-col lg:flex-row gap-2">
        <Accounts />
        <div className="flex-1 overflow-y-auto sleek_scrollbar px-2">
          <AccountDetails />
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
