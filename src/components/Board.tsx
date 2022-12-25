import { Template, Vec2 } from "../main";
import { Grid } from "../components";

interface BoardProps {
  current: Template;
  currentOffset: Vec2;
  board: string[][];
}

export default ({ current, currentOffset, board }: BoardProps) => {
  current.offsets.forEach((offset) => {
    board[offset.y + currentOffset.y][offset.x + currentOffset.x] =
      current.type;
  });

  return <Grid grid={board} />;
};
