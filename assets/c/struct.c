struct Point {
  int x;
  int y;
};

int printf(char *fmt, int c);

int main() {
  struct Point p;
  struct Point *pp;

  pp = &p;

  p.x = 17;
  pp->y = 31;

  printf("p.x: %d, ", p.x);
  printf("p.y: %d, ", p.y);
  printf("pp->x: %d, ", pp->x);
  printf("pp->y: %d, ", pp->y);

  return 0;
}
