import { describe, it } from 'mocha';
import { expect } from 'chai';
import { isUnique } from './IsUnique';

describe('isUnique', function () {
  it('should return true for a unique string', function () {
    expect(isUnique('abcde')).to.be.true;
  });
  it('should return false for a non-unique string', function () {
    expect(isUnique('abcdee')).to.be.false;
  });
});
