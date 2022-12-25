import { Template, Tile, Vec2 } from "../main";
import { Grid } from "../components";

interface NextProps {
  next: Template;
}

export default ({ next }: NextProps) => {
  const width = next.type === Tile.I ? 4 : 3;
  const origin: Vec2 = { x: width >> 1, y: width >> 1 };
  const grid = Array.from(Array(width), (_) =>
    Array(width).fill(Tile.EMPTY_DARK)
  );

  next.offsets.forEach((offset) => {
    grid[origin.y + offset.y][origin.x + offset.x] = next.type;
  });

  return <Grid grid={grid} />;
};
