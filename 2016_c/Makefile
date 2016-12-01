ALL: point_map.o point_map_test day_1

point_map.o: src/point_map.c src/point_map.h
	gcc $< -c -o $@ -g -Wall

point_map_test: test/point_map.c point_map.o
	gcc $^ -o $@ -g -Wall

day_1: src/day_1.c point_map.o
	gcc $^ -o $@ -g -Wall

clean:
	rm -f day_1 point_map.o point_map_test
