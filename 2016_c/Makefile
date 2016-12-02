CC:=gcc -std=c99

ALL: point_map_test day_1 day_2 day_2_part_2

point_map.o: src/point_map.c src/point_map.h
	$(CC) $< -c -o $@ -g -Wall

common.o: src/common.c src/common.h
	$(CC) $< -c -o $@ -g -Wall

point_map_test: test/point_map.c point_map.o
	$(CC) $^ -o $@ -g -Wall

day_1: src/day_1.c point_map.o common.o
	$(CC) $^ -o $@ -g -Wall

day_2: src/day_2.c common.o
	$(CC) $^ -o $@ -g -Wall

day_2_part_2: src/day_2_part_2.c common.o
	$(CC) $^ -o $@ -g -Wall

clean:
	rm -f day_1 point_map.o point_map_test common.o day_2 day_2_part_2
