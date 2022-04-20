testEditDistance:
	javac -d classes/ EditDistance.java
	java -ea -cp classes/ TestEditDistance

testEditDistanceRust:
	rustc --test -o bin/edit_distance edit_distance.rs
	./bin/edit_distance

