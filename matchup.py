import numpy as np
import pandas as pd
data = pd.read_csv(r"C:\Users\Chris\OneDrive\Documents\Code\teams.csv")
data.head()
Teams = list(data["Teams"])
data = pd.read_csv(r"C:\Users\Chris\OneDrive\Documents\Code\matches.csv")
data.head()
Blue_score = list(data["Blue Score"])
Red_score = list(data["Red Score"])
Y = []
for i in Blue_score:
    Y += [[i]]
for i in Red_score:
    Y += [[i]]
Team1 = list(data["Blue Team 1"])
Team2 = list(data["Blue Team 2"])
Team3 = list(data["Blue Team 3"])
Team4 = list(data["Red Team 1"])
Team5 = list(data["Red Team 2"])
Team6 = list(data["Red Team 3"])
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
Teams = [1] + Teams
score = np.dot(np.linalg.inv(np.dot(X.transpose(), X)), np.dot(X.transpose(), Y))
for i in range(len(Teams)):
    print(Teams[i], score[i])
