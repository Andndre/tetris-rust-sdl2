import { Link } from "react-router-dom";
import tetrisImg from "../assets/tetris.png";
import { Page } from "../components";

export default function Home() {
  return (
    <Page>
      <div className="flex flex-col w-screen h-screen items-center justify-center gap-2">
        <img src={tetrisImg} alt="tetris" />
        <div className="py-3"></div>

        <Link to="play" relative="path">
          <div className="btn w-48 text-center">Play</div>
        </Link>
        <button className="w-48">Settings</button>
        <p>
          <strong>High score: </strong>
        </p>
      </div>
    </Page>
  );
}
