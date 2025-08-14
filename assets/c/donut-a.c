int printf(char *fmt, int c);

int m(int a, int b) { return (a * b + 5000) / 10000; }

int a(int *c, int *s, int d, int t) {
  printf("c:%d,", *c);
  printf("s:%d,", *s);
  printf("d:%d,", d);
  printf("t:%d,", t);
  int k;
  int l;
  k = m(*c, d) - m(*s, t);
  l = m(*s, d) + m(*c, t);
  printf("k:%d,", k);
  printf("l:%d,", l);
  printf("%c", 10);
  *c = k;
  *s = l;

  return 0;
}

int main() {
  int c;
  int s;
  c = 143;
  s = 834;

  int res;

  a(&c, &s, 321, 976);
  printf("c:%d,", c);
  printf("s:%d,", s);
  printf("%c", 10);

  return 0;
}
