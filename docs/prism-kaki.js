(function(Prism) {
  Prism.languages.kaki = {
    'comment': [
      {
        pattern: /#(?!\[\[).*/,
        lookbehind: true
      },
      {
        pattern: /#\[\[[\s\S]*\]\]/,
        lookbehind: true
      }
    ],
    'keyword': /\b(_|abstract|break|cons|continue|else|for|func|if|in|loop|pub|return|self|Self|trait|type|use|while)\b/,
    'function': /@?@?_*[a-z][a-z0-9_]*(!|\?)?(?=\s*\()/,
    'field': {
      pattern: /@?@_*[a-z][a-z0-9_]*(!|\?)?/,
      alias: 'property'
    },
    'constant': {
      pattern: /\b(_*[A-Z][A-Z_0-9]*)\b/,
      alias: 'number'
    },
    'type-trait': {
      pattern: /\b(_*[A-Z][A-Za-z_0-9]*)\b/,
      alias: 'variable'
    },
    'text': {
      pattern: /\b(_*[a-z][a-z0-9_]*(!|\?)?)/,
      alias: 'entity'
    },
    'anonymous-function-arg': {
      pattern: /\$\d*/,
      alias: 'entity'
    },
    'boolean': /\b(false|none|true)\b/,
    'number': [
      /\b(0b[01](_?[01])*)\b/,
      /\b(0o[0-7](_?[0-7])*)\b/,
      /\b(0x[\dA-Fa-f](_?[\dA-Fa-f])*)\b/,
      /\b((\d(_?\d)*\.)?(\d(_?\d)*)([e][+-]?\d(_?\d)*)?)\b/
    ],
    'operator': /[\~\-\%\+\^\,]|\*\*?|\/\/?|<(<|=)?|>(>|=)?|==?|!=?|&&?|\|\|?|\?=/,
    'punctuation': /(\(|\)|\[|\]|\{|\}|;|::?|,|\.\.\.|\.|\?)/,
    'string': {
      pattern: /\@?"(\\|(?!")[\s\S])*"/,
      greedy: true,
      inside: {
        interpolation: {
          pattern: /\\(n|r|t|\\|0|"|u\{[0-9a-fA-F]{1,6}\})/,
          alias: 'number'
        }
      }
    },
    'line-continue': {
      pattern: /\\/,
      alias: 'keyword'
    }
  };
}(Prism));
