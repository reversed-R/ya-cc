int main(){
  int x;
  int y;
  int *z;
  
  x = 1;
  y = 2;
  z = &y + 1;
  *z = 3;
  return x;
}
