import { Link } from "react-router-dom";
import { Page } from "../components";
import tetrisImg from "../assets/tetris.png";

export default () => {
  return (
    <Page>
      <div className="flex flex-col w-screen h-screen items-center justify-center gap-2">
        <img src={tetrisImg} alt="tetris" />
        <div className="py-3"></div>

        <Link to="play" relative="path">
          <div className="btn w-48 text-center">PLAY</div>
        </Link>
        <button className="w-48">SETTINGS</button>
        <p>
          <strong>High score: </strong>
        </p>
      </div>
    </Page>
  );
};
