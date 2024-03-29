/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package dev.chagnon.aoc2023;

import java.security.InvalidParameterException;
import dev.chagnon.aoc2023.days.Day01;
import dev.chagnon.aoc2023.days.Day02;
import dev.chagnon.aoc2023.days.Day03;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NonNull;

@AllArgsConstructor
@Getter
public enum Day {
    ONE(1, new Day01()), TWO(2, new Day02()), THREE(3, new Day03());

    private final int n;

    @NonNull
    private final DayRunner runner;

    public static Day fromString(String s) {
        int n = Integer.parseInt(s);
        return switch (n) {
            case 1 -> Day.ONE;
            case 2 -> Day.TWO;
            case 3 -> Day.THREE;
            case 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 -> {
                throw new UnsupportedOperationException("Day not yet implemented");
            }
            default -> {
                throw new InvalidParameterException(String.format("Invalid day: %d", n));
            }
        };
    }
}
