import numpy as np
import pandas as pd
data = pd.read_csv(r"C:\Users\Chris\OneDrive\Documents\Code\teams.csv")
data.head()
Teams = list(data["Teams"])
data = pd.read_csv(r"C:\Users\Chris\OneDrive\Documents\Code\matches.csv")
data.head()
Blue_score = list(data["Blue Score"])
Blue_A_GP = list(data["Blue Auto Game Piece Count"])
Blue_A_P = list(data["Blue Auto Points"])
Blue_T_GP = list(data["Blue Teleop Game Piece Count"])
Blue_T_P = list(data["Blue Teleop Points"])
Red_score = list(data["Red Score"])
Red_A_GP = list(data["Red Auto Game Piece Count"])
Red_A_P = list(data["Red Auto Points"])
Red_T_GP = list(data["Red Teleop Game Piece Count"])
Red_T_P = list(data["Red Teleop Points"])
Y = []
for i in range(len(Blue_score)):
    Y += [[Blue_score[i], Blue_A_GP[i], Blue_A_P[i], Blue_T_GP[i], Blue_T_P[i]]]
for i in range(len(Blue_score)):
    Y += [[Red_score[i], Red_A_GP[i], Red_A_P[i], Red_T_GP[i], Red_T_P[i]]]
Team1 = list(data["Blue Team 1"])
Team2 = list(data["Blue Team 2"])
Team3 = list(data["Blue Team 3"])
Team4 = list(data["Red Team 1"])
Team5 = list(data["Red Team 2"])
Team6 = list(data["Red Team 3"])
X = []
for i in range(len(Blue_score)):
    temp = [0] * len(Teams)
    temp[Teams.index(Team1[i])] += 1
    temp[Teams.index(Team2[i])] += 1
    temp[Teams.index(Team3[i])] += 1
    X += [temp]
for i in range(len(Red_score)):
    temp = [0] * len(Teams)
    temp[Teams.index(Team4[i])] += 1
    temp[Teams.index(Team5[i])] += 1
    temp[Teams.index(Team6[i])] += 1
    X += [temp]
Y = np.matrix(Y)
X = np.matrix(X)
score = np.asarray(np.dot(np.linalg.inv(np.dot(X.transpose(), X)), np.dot(X.transpose(), Y)))
Avrage = np.sum(score, axis=0)/len(Teams)
print(Avrage)
Scores = []
for i in range(len(Teams)):
    Scores += [sum(score[i] / Avrage)]
    print(Teams[i], score[i] / Avrage)
print(Scores)
