interface GridProps {
  grid: string[][];
}

export default function Grid(props: GridProps) {
  return (
    <div className="flex flex-col gap-1">
      {props.grid.map((row, i) => {
        return (
          <div key={"row-" + i} className="flex gap-1">
            {row.map((cell, j) => {
              return (
                <div
                  key={"col-" + j}
                  className="h-6 w-6 rounded"
                  style={{ background: cell }}
                ></div>
              );
            })}
          </div>
        );
      })}
    </div>
  );
}
