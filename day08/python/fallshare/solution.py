

def main():

    file = open('input.txt', 'r')
    rows = file.read().split("\n")
    map = []

    for row in rows:
        row = [int(x) for x in row]
        map.append(row)

    visibleTrees = []
    highest_scenic_score = 0
    for row in range(len(map)):
        for column in range(len(map[0])):
            print(".")
            vectors = [[1,0],[-1,0],[0,1],[0,-1]]
            
            scenic_score = 1
            visible = False
            for vector in vectors:
                
                current_row = row + vector[0]
                current_column = column + vector[1]    
                visibleInDirection = True
                view_width = 1
                while(      (current_row  >= 0) 
                        and (current_row  <= len(map) - 1)
                        and (current_column >= 0) 
                        and (current_column <= len(map[0]) - 1)):

                    # print(f"cur col {current_column} - cur row {current_row}")

                    if map[row][column] <= map[current_row][current_column]:
                        # print(f"{row}-{column}:[{map[row][column]}] not visible due to {current_row}-{current_column}:[{map[current_row][current_column]}]")
                        visibleInDirection = False
                        break

                    current_row += vector[0]
                    current_column += vector[1]
                    view_width += 1

                if visibleInDirection:
                    visible = True
                scenic_score *= view_width
                print(f"sc: {scenic_score} vw: {view_width}")

            if visible:
                #print(f"{map[row][column]} is visible")
                visibleTrees.append([row,column])
            if scenic_score >= highest_scenic_score:
                highest_scenic_score = scenic_score

    
   
    print(f"{len(visibleTrees)} trees can be seen.")
    print(f"Highest scenic score is {highest_scenic_score}")    #print(visibleTrees)


            


if __name__ == '__main__':
    main()