int putchar(char c);
int *memset(void *buf, int c, int n);
int usleep(int usec);
int printf(char *fmt, int c);

int m(int a, int b) { return (a * b + 5000) / 10000; }
int m0(int a, int b) { return (a * b + 5000) / 10000; }
int m1(int a, int b) { return (a * b + 5000) / 10000; }
int m2(int a, int b) { return (a * b + 5000) / 10000; }
int m4(int a, int b) { return (a * b + 5000) / 10000; }
int m5(int a, int b) { return (a * b + 5000) / 10000; }
int m6(int a, int b) { return (a * b + 5000) / 10000; }
int m7(int a, int b) { return (a * b + 5000) / 10000; }
int m8(int a, int b) { return (a * b + 5000) / 10000; }

int a(int *c, int *s, int d, int t) {
  int k;
  int l;
  k = m(*c, d) - m(*s, t);
  l = m(*s, d) + m(*c, t);
  *c = k;
  *s = l;

  return 0;
}

int main3(int **vars, int *z, char *b) {
  int *ps = vars[0];
  int *pq = vars[1];
  int *pr = vars[2];
  int *pu = vars[3];
  int *pv = vars[4];

  int l;
  int p;
  l = 0;
  p = *ps;

  int i;
  i = 0;
  while (i < 88) {
    int w;
    int e;
    w = 0;
    e = *ps;

    int j;
    j = 0;
    while (j < 314) {
      int f;
      int g;
      int t;
      int x;
      int y;
      int o;
      int N;

      f = p + 2 * *ps;
      g = *ps * *ps / (m0(m0(w, f), *pr) + m0(l, *pq) + 5 * *ps);
      t = m1(m1(w, *pq), f) - m1(l, *pr);
      x = 40 + 30 * m2(g, m2(m2(e, *pu), f) - m2(t, *pv)) / *ps;
      y = 12 + 15 * m4(g, m4(m4(e, *pv), f) + m4(t, *pu)) / *ps;
      o = x + 80 * y;
      N = 8 *
          (m5(m5(l, *pr) - m5(m5(w, *pq), p), *pu) - m5(m5(w, *pr), p) -
           m5(l, *pq) - m6(m6(e, *pv), p)) /
          *ps;
      if (y > 0) {
        if (g > z[o]) {
          if (22 > y) {
            if (x > 0) {
              if (80 > x) {
                z[o] = g;
                if (N >= 1) {
                  b[o] = ".,-~:;=!*#$@"[N];
                } else {
                  b[o] = ".,-~:;=!*#$@"[0];
                }
              }
            }
          }
        }
      }
      j = j + 1;
      a(&e, &w, *ps - 2, 200);
    }
    i = i + 1;
    a(&p, &l, 9974 + i % 2, 714);
  }
  printf("%c[H", 27);
  int k;
  k = 0;
  while (k < 1761) {
    if (k % 80) {
      putchar(b[k]);
    } else {
      printf("%c", 10);
    }
    k = k + 1;
  }
  usleep(5 * *ps);
  a(&*pq, &*pr, *ps - 8, 400);
  a(&*pu, &*pv, *ps - 2, 200);

  return 0;
}

int main2() {
  int z[1760];
  char b[1760];

  int s;
  int q;
  int r;
  int u;
  int v;
  s = 10000;
  q = s;
  r = 0;
  u = s;
  v = 0;

  int *vars[5];

  while (1) {
    memset(b, 32, 1760);
    memset(z, 0, 1760 * sizeof(q));

    vars[0] = &s;
    vars[1] = &q;
    vars[2] = &r;
    vars[3] = &u;
    vars[4] = &v;
    main3(vars, z, b);
  }
  return 0;
}

int main() {
  printf("%c[2J", 27);
  main2();

  return 0;
}
