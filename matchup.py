import numpy as np
import pandas as pd
data = pd.read_csv(r"teams.csv")
data.head()
Teams = data["Teams"]
data = pd.read_csv(r"mathces.csv")
data.head()
Blue_score = [100, 80]
Red_score = [80, 100]
Y = []
for i in Blue_score:
    Y += [[i]]
for i in Red_score:
    Y += [[i]]
Team1 = [1, 3]
Team2 = [2, 4]
Team3 = [5, 6]
Team4 = [7, 8]
Team5 = [1, 3]
Team6 = [5, 9]
X = []
for i in range(len(Blue_score)):
    temp = [1] + [0] * len(Teams)
    temp[Teams.index(Team1[i]) + 1] += 1
    temp[Teams.index(Team2[i]) + 1] += 1
    temp[Teams.index(Team3[i]) + 1] += 1
    X += [temp]
for i in range(len(Red_score)):
    temp = [1] + [0] * len(Teams)
    temp[Teams.index(Team4[i]) + 1] += 1
    temp[Teams.index(Team5[i]) + 1] += 1
    temp[Teams.index(Team6[i]) + 1] += 1
    X += [temp]
Y = np.matrix(Y)
X = np.matrix(X)
np.dot(np.linalg.inv(np.dot(X.transpose(), X)), np.dot(X.transpose(), Y))
