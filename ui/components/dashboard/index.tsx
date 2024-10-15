import React from "react";
import Accounts from "./accounts";
import AccountDetails from "./account-details";

const Dashboard = () => {
  return (
    <div className="w-full">
      <h1 className="text-2xl">Dashboard</h1>
      <div className="mt-4 h-[80vh] flex flex-col lg:flex-row">
        <Accounts />
        <div className="flex-1">
          <AccountDetails />
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
