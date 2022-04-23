import { Link } from "react-router-dom";

const Guide = () => {
  return (
    <main className="App">
      <header>
        <Link to="/" className="home-link">
          <div className="header-titles">
            <h1>Some game name</h1>
            <p>Only you can win i guess</p>
          </div>
        </Link>
      </header>

      <div className="score-board-container">
        <h3>How to play</h3>

        <div>
          <span className="help">To be decide</span>
        </div>
      </div>
    </main>
  );
};

export default Guide;
