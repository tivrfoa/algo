class Solution(object):
    def minDistance(self, word1, word2):
        """
        :type word1: str
        :type word2: str
        :rtype: int
        """
        return fastest(word1, word2)


def edDistRecursiveMemo(x, y, memo=None):
    ''' A version of edDistRecursive with memoization.  For each x, y we see, we
        record result from edDistRecursiveMemo(x, y).  In the future, we retrieve
        recorded result rather than re-run the function. '''
    if memo is None: memo = {}
    if len(x) == 0: return len(y)
    if len(y) == 0: return len(x)
    if (len(x), len(y)) in memo:
        return memo[(len(x), len(y))]
    delt = 1 if x[-1] != y[-1] else 0
    diag = edDistRecursiveMemo(x[:-1], y[:-1], memo) + delt
    vert = edDistRecursiveMemo(x[:-1], y, memo) + 1
    horz = edDistRecursiveMemo(x, y[:-1], memo) + 1
    ans = min(diag, vert, horz)
    memo[(len(x), len(y))] = ans
    return ans



from numpy import zeros

def edDistDp(x, y):
    """ Calculate edit distance between sequences x and y using
        matrix dynamic programming.  Return distance. """
    D = zeros((len(x)+1, len(y)+1), dtype=int)
    D[0, 1:] = range(1, len(y)+1)
    D[1:, 0] = range(1, len(x)+1)
    for i in range(1, len(x)+1):
        for j in range(1, len(y)+1):
            delt = 1 if x[i-1] != y[j-1] else 0
            D[i, j] = min(D[i-1, j-1]+delt, D[i-1, j]+1, D[i, j-1]+1)
    return D[len(x), len(y)]


from collections import deque

def fastest(word1, word2):
    """
    :type word1: str
    :type word2: str
    :rtype: int
    """
    if word1 == word2: return 0

    #this is the line of code for repeatedly checking if the endings match
    while len(word1) and len(word2) and word1[-1]==word2[-1]:
        word1, word2 = (word1[:-1], word2[:-1])


    #everything in the queue is a tuple
    #(word1, word2, depth)
    QVisited = deque()
    QVisited.append((word1, word2, 0))

    #this is just like the queue except the tuples don't have the depth
    SVisited = set((word1, word2))


    #main loop
    while True:
        w1, w2, depth = QVisited.popleft()

        #empty cases
        if len(w1) == 0:
            w2 = w2[:-1]
            if w1 == w2: return depth + 1
            if (w1, w2) not in SVisited:
                SVisited.add((w1, w2))
                QVisited.append((w1, w2, depth + 1))
            continue
        if len(w2) == 0:
            w1 = w1[:-1]
            if w1 == w2: return depth + 1
            if (w1, w2) not in SVisited:
                SVisited.add((w1, w2))
                QVisited.append((w1, w2, depth + 1))
            continue

        #Recursive Case 1: shorten w1
        new_w1 = w1[:-1]
        new_w2 = w2
        if new_w1 == new_w2: return depth + 1
        while len(new_w1) and len(new_w2) and new_w1[-1]==new_w2[-1]:
            new_w1, new_w2 = (new_w1[:-1], new_w2[:-1])

        if (new_w1, new_w2) not in SVisited:
            SVisited.add((new_w1, new_w2))
            QVisited.append((new_w1, new_w2, depth + 1))

        #Recursive Case 2: shorten w2
        new_w1 = w1
        new_w2 = w2[:-1]
        if new_w1 == new_w2: return depth + 1
        while len(new_w1) and len(new_w2) and new_w1[-1]==new_w2[-1]:
            new_w1, new_w2 = (new_w1[:-1], new_w2[:-1])

        if (new_w1, new_w2) not in SVisited:
            SVisited.add((new_w1, new_w2))
            QVisited.append((new_w1, new_w2, depth + 1))

        #Recursive Case 2: shorten both
        new_w1 = w1[:-1]
        new_w2 = w2[:-1]
        if new_w1 == new_w2: return depth + 1
        while len(new_w1) and len(new_w2) and new_w1[-1]==new_w2[-1]:
            new_w1, new_w2 = (new_w1[:-1], new_w2[:-1])

        if (new_w1, new_w2) not in SVisited:
            SVisited.add((new_w1, new_w2))
            QVisited.append((new_w1, new_w2, depth + 1))


assert 7 == Solution().minDistance("auiabbbaiii", "abba")

