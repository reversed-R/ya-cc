#include <stdio.h>  // putchar(), printf()
#include <string.h> // memset()
#include <unistd.h> // usleep()
/* int putchar(char c); */
/* int *memset(void *buf, int c, int n); */
/* int usleep(int usec); */
/* int printf(char *fmt, int c); */
/* int printf(char *fmt); */

int m(int a, int b) { return (a * b + 5000) / 10000; }

int a(int *c, int *s, int d, int t) {
  int k;
  int l;
  k = m(*c, d) - m(*s, t);
  l = m(*s, d) + m(*c, t);
  *c = k;
  *s = l;

  return 0;
}

int main() {
  int z[1760];
  char b[1760];
  /* printf("%c[2J", 27); */
  printf("\033[2J");
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

  while (1) {
    memset(b, 32, 1760);
    memset(z, 0, 1760 * sizeof(q));
    int l;
    int p;
    l = 0;
    p = s;

    int i;
    i = 0;
    while (i < 88) {
      int w;
      int e;
      w = 0;
      e = s;

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

        f = p + 2 * s;
        g = s * s / (m(m(w, f), r) + m(l, q) + 5 * s);
        t = m(m(w, q), f) - m(l, r);
        x = 40 + 30 * m(g, m(m(e, u), f) - m(t, v)) / s;
        y = 12 + 15 * m(g, m(m(e, v), f) + m(t, u)) / s;
        o = x + 80 * y;
        N = 8 *
            (m(m(l, r) - m(m(w, q), p), u) - m(m(w, r), p) - m(l, q) -
             m(m(e, v), p)) /
            s;
        if (y > 0 && g > z[o] && 22 > y && x > 0 && 80 > x) {
          z[o] = g;
          if (N >= 1) {
            b[o] = ".,-~:;=!*#$@"[N];
          } else {
            b[o] = ".,-~:;=!*#$@"[0];
          }
        }
        /* if (y > 0) { */
        /*   if (g > z[o]) { */
        /*     if (22 > y) { */
        /*       if (x > 0) { */
        /*         if (80 > x) { */
        /*           z[o] = g; */
        /*           if (N >= 1) { */
        /*             b[o] = ".,-~:;=!*#$@"[N]; */
        /*           } else { */
        /*             b[o] = ".,-~:;=!*#$@"[0]; */
        /*           } */
        /*         } */
        /*       } */
        /*     } */
        /*   } */
        /* } */
        j = j + 1;
        a(&e, &w, s - 2, 200);
      }
      i = i + 1;
      a(&p, &l, 9974 + i % 2, 714);
    }
    /* printf("%c[H", 27); */
    printf("\033[H");
    int k;
    k = 0;
    while (k < 1761) {
      if (k % 80) {
        putchar(b[k]);
      } else {
        /* printf("%c", 10); */
        putchar(10);
      }
      k = k + 1;
    }
    usleep(5 * s);
    a(&q, &r, s - 8, 400);
    a(&u, &v, s - 2, 200);
  }
  return 0;
}
