int printf(char *fmt, int c);

int m(int a, int b) {
  int n;
  int m;
  n = a * b + 5000;
  m = n / 10000;

  printf("(a * b + 5000) / 10000 : %d", m);
  printf("%c", 10);

  return (a * b + 5000) / 10000;
}

int main() {
  int c;
  int s;
  c = 143;
  s = 834;

  int res;

  res = m(c, s);

  printf("res: %d", res);
  printf("%c", 10);

  return 0;
}
