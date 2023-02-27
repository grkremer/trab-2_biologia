import pandas as pd

def encontraMenorDistancia(matrix):
  min = matrix.iloc[0,-1]
  for i in matrix.iterrows():
    for j in matrix:
      if (i[1][j] <= min and i[1][j] > 0):
        min = i[1][j]
        gen1 = i[0]
        gen2 = j
  return min, (gen1, gen2)

def substituiPar(matrix, par):
    print(matrix)
    matrix[str(par).replace("'",'')] = matrix.apply(lambda x: (x[par[0]] + x[par[1]])/2)
    matrix.drop(list(par), axis =1, inplace=True)
    matrix.drop(list(par), inplace = True)
    vec = list(matrix[str(par).replace("'",'')])
    vec.append(0)
    matrix.loc[str(par).replace("'",'')] = vec
    print(min)
    

def upmga(matrix):
  while len(matrix.columns) > 1:
    min, par = encontraMenorDistancia(matrix)
    substituiPar(matrix, par)
  return (matrix.columns[0])

matrix = pd.read_csv('matriz_distancias.csv', sep = '\t', index_col= 'index')
resultado = upmga(matrix)
print(resultado)
