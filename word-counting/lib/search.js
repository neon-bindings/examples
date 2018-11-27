function nthIndexOf(haystack, needle, n) {
  let index = -1;
  for (let i = 0; i < n; i++) {
    index = haystack.indexOf(needle, index + 1);
    if (index < 0) {
      return -1;
    }
  }
  return index;
}

function lines(corpus) {
  return corpus
    .split(/\n+/)
    .map(line => line.substring(nthIndexOf(line, ',', 3) + 1));
}

function skipPunc(word) {
  // eslint-disable-next-line
  for (var i = 0, n = word.length; i < n; i++) {
    if (/[a-zA-Z]/.test(word[i])) {
      break;
    }
  }
  // eslint-disable-next-line
  return word.substring(i);
}

function matches(word, search) {
  const start = skipPunc(word);
  let i = 0;
  const m = start.length;
  const n = search.length;
  if (m < n) {
    return false;
  }
  while (i < n) {
    if (start[i].toLowerCase() !== search[i]) {
      return false;
    }
    i++;
  }
  return i === m || !/[a-zA-Z]/.test(start[i]);
}

function wcLine(line, search) {
  const words = line.split(' ');
  let total = 0;
  for (let i = 0, n = words.length; i < n; i++) {
    if (matches(words[i], search)) {
      total++;
    }
  }
  return total;
}

exports.search = function search(corpus, _search) {
  const ls = lines(corpus);
  let total = 0;
  for (let i = 0, n = ls.length; i < n; i++) {
    total += wcLine(ls[i], _search);
  }
  return total;
};
