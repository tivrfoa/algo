testEditDistance:
	javac -d classes/ EditDistance.java
	java -ea -cp classes/ TestEditDistance

testEditDistanceRust:
	rustc --test -o bin/edit_distance edit_distance.rs
	./bin/edit_distance

testEditDistanceRustV2:
	rustc --test -o bin/edit_distance_v2 edit_distance_v2.rs
	./bin/edit_distance_v2

testEditDistanceElegantDP:
	rustc --test -o bin/edit_distance_elegant_dp edit_distance_elegant_dp.rs
	./bin/edit_distance_elegant_dp

testFenwickTreeRangeQueryPointUpdate:
	rustc --test -o bin/FenwickTreeRangeQueryPointUpdate FenwickTreeRangeQueryPointUpdate.rs
	./bin/FenwickTreeRangeQueryPointUpdate
