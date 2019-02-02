defmodule Point do
  defstruct x: 0, y: 0, vx: 0, vy: 0

  def from_string(str) do
    [x | [y | [vx | [vy | _]]]] =
      Regex.scan(~r/-?\d+/, str)
      |> List.flatten()
      |> Enum.map(&String.to_integer/1)

    %Point{x: x, y: y, vx: vx, vy: vy}
  end

  def move(%Point{x: x, y: y, vx: vx, vy: vy} = point) do
    %{point | x: x + vx, y: y + vy}
  end

  def bounds(points) do
    xs = Enum.map(points, & &1.x)
    ys = Enum.map(points, & &1.y)

    %{
      top_left: %{x: Enum.min(xs), y: Enum.min(ys)},
      bottom_right: %{x: Enum.max(xs), y: Enum.max(ys)}
    }
  end

  def all_connected(points) do
    points
    |> Enum.all?(fn point ->
      Enum.any?(points, fn other ->
        (point.x == other.x && abs(point.y - other.y) == 1) ||
        (point.y == other.y && abs(point.x - other.x) == 1) ||
        (abs(point.y - other.y) == 1 && abs(point.x - other.x) == 1)
      end)
    end)
  end

  def plot_points(points, bounds) do
    for y <- bounds.top_left.y..bounds.bottom_right.y do
      for x <- bounds.top_left.x..bounds.bottom_right.x do
        if Enum.any?(points, fn pt -> pt.x == x && pt.y == y end) do
          IO.write("#")
        else
          IO.write(".")
        end
      end
      IO.write("\n")
    end
  end

  def run(points), do: run(points, 0)
  def run(points, seconds) do
    if all_connected(points) do
      plot_points(points, bounds(points))
      IO.puts("Took #{seconds} seconds")
    else
      points
      |> Enum.map(&Point.move/1)
      |> run(seconds + 1)
    end
  end
end

points =
  File.read!("input.txt")
  |> String.trim()
  |> String.split("\n")
  |> Enum.map(&Point.from_string/1)

Point.run(points)
