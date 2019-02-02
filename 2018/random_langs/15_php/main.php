<?php
  class Point {
    public $x;
    public $y;

    function __construct($x, $y)
    {
      $this->x = $x;
      $this->y = $y;
    }
  }

  class Fighter
  {
    public $health = 200;
    public $power = 3;
    public $position;
    public $bf;

    function __construct($x, $y, $bf)
    {
      $this->position = new Point($x, $y);
      $this->bf = $bf;
    }

    public function adjacentCells()
    {

    }
  }

  class Elf extends Fighter {}
  class Goblin extends Fighter {}

  class Battlefield
  {
    public $grid = array();
    public $fighters = array();

    function __construct($text)
    {
      $lines = explode(PHP_EOL, $text);

      foreach ($lines as $y => $line) {
        $line = str_split($line);
        $current = array();

        foreach ($line as $x => $char) {
          if ($char == 'G') {
            array_push($this->fighters, new Goblin($x, $y, $this));
          } elseif ($char == 'E') {
            array_push($this->fighters, new Elf($x, $y, $this));
          }

          if ($char == '#') {
            array_push($current, '#');
          } else {
            array_push($current, '.');
          }
        }

        array_push($this->grid, $current);
      }
    }
  }

  $input = fopen("input.txt", "r");
  $input = fread($input, filesize("input.txt"));

  $bf = new Battlefield($input);
  echo count($bf->fighters)
?>
