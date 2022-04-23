import App from "./App";
import React from "react";
import ReactDOM from "react-dom";

import { Route, Routes, BrowserRouter } from "react-router-dom";
import "./index.css";

// We import the pages we just created
import Play from "./pages/play";
import Guide from "./pages/guide";
import Leaderboard from "./pages/leaderboard";
import { getChainOptions, WalletProvider } from "@terra-money/wallet-provider";

getChainOptions().then((chainOptions) => {
  ReactDOM.render(
    <React.StrictMode>
      <WalletProvider {...chainOptions}>
        <div className="App-header">
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<App />} />
              <Route path="/play" element={<Play />} />
              <Route path="/leaderboard" element={<Leaderboard />} />
              <Route path="/guide" element={<Guide />} />
            </Routes>
          </BrowserRouter>
        </div>
      </WalletProvider>
    </React.StrictMode>,
    document.getElementById("root")
  );
});
