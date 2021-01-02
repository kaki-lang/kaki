(function(Prism) {
  Prism.languages.kaki = {
    'comment': [
      {
        pattern: /\/\/\/.*/,
        lookbehind: true
      },
      {
        pattern: /#.*/,
        lookbehind: true
      }
    ],
    'keyword': /\b(_|abstract|alias|break|cons|continue|else|for|fn|if|in|loop|pub|return|self|Self|trait|type|use|where|while)\b/,
    'boolean': /\b(false|none|true)\b/,
    'function': /@?@?_*[a-z][a-z0-9_]*(!|\?)?(?=\s*\()/,
    'field': {
      pattern: /@?@_*[a-z][a-z0-9_]*(!|\?)?/,
      alias: 'property'
    },
    'number': [
      /\b(0b[01](_?[01])*)\b/,
      /\b(0o[0-7](_?[0-7])*)\b/,
      /\b(0x[\dA-Fa-f](_?[\dA-Fa-f])*)\b/,
      /\b((\d(_?\d)*\.)?(\d(_?\d)*)([eE][+-]?\d(_?\d)*)?)\b/
    ],
    'constant': {
      pattern: /(?<![a-z])_*[A-Z][A-Z_0-9]*(?![a-z])(!|\?)?/,
      alias: 'number'
    },
    'type-trait': {
      pattern: /_*[A-Z][A-Za-z_0-9]*(!|\?)?/,
      alias: 'variable'
    },
    'variable': /_*[a-z][a-z0-9_]*(!|\?)?/,
    'anonymous-function-arg': {
      pattern: /_\d*/,
      alias: 'entity'
    },
    'operator': /[\~\-\%\+\^\,]|\*\*?|\/\/?|<(<|=)?|>(>|=)?|==?|!=?|&&?|\|\|?|\?=/,
    'punctuation': /(\(|\)|@?\[|\]|@?\{|\}|;|::?|,|\.|\?)/,
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
