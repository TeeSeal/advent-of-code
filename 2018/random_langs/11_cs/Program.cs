using System;
using System.Collections.Generic;

namespace _11_cs
{

  public struct Point
  {
    public int x, y;

    public Point(int x, int y)
    {
      this.x = x;
      this.y = y;
    }
  }

  public class Grid
  {
    public int[][] cells;
    public int serialNumber;

    public Grid(int serialNumber)
    {
      this.serialNumber = serialNumber;
      cells = new int[300][];

      for (int y = 0; y < 300; y++)
      {
        cells[y] = new int[300];
        for (int x = 0; x < 300; x++)
        {
          int rackId = x + 11;
          cells[y][x] = (rackId * (y + 1) + serialNumber) * rackId / 100 % 10 - 5;
        }
      }
    }

    public int powerForSquareAt(Point pt, int size)
    {
      int power = 0;

      for (int y = pt.y; y < pt.y + size; y++)
        {
          for (int x = pt.x; x < pt.x + size; x++) power += cells[y][x];
        }

      return power;
    }

    public (Point, int) maxPowerSquareLocation(int size)
    {
      Point current = new Point(0, 0);
      int max = powerForSquareAt(current, size);

      for (int y = 0; y < 300 - size; y++)
      {
        for (int x = 0; x < 300 - size; x++)
        {
          Point pt = new Point(x, y);
          int power = powerForSquareAt(pt, size);

          if (power > max)
          {
            max = power;
            current = pt;
          }
        }
      }

      return (current, max);
    }
  }

  class Program
  {
    static void Main(string[] args)
    {
      var grid = new Grid(6042);
      var (coords, max) = grid.maxPowerSquareLocation(3);
      Console.WriteLine("Part 1: {0},{1}", coords.x + 1, coords.y + 1);

      var maxSize = 1;
      for (int size = 1; size <= 300; size++)
      {
        Console.WriteLine(size);
        var (newCoords, newMax) = grid.maxPowerSquareLocation(size);
        if (newMax > max)
        {
          max = newMax;
          maxSize = size;
          coords = newCoords;
        }
      }

      Console.WriteLine("Part 2: {0},{1},{2}", coords.x + 1, coords.y + 1, maxSize);
    }
  }
}
