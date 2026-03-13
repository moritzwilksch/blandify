import inspect

import blandify
import pytest


class TestPublicApi:
    """Tests for the public Python API surface."""

    def test_module_has_docstring(self):
        assert inspect.getdoc(blandify)

    def test_normalize_has_docstring(self):
        assert inspect.getdoc(blandify.normalize)

    def test_removed_classes_are_not_exposed(self):
        assert not hasattr(blandify, "Categories")
        assert not hasattr(blandify, "NormalizerConfig")
        assert not hasattr(blandify, "Normalizer")


class TestNormalizeDefaults:
    """Tests for default normalize() behavior."""

    def test_basic_usage(self):
        assert blandify.normalize("\u201chello\u201d") == '"hello"'

    def test_empty_string(self):
        assert blandify.normalize("") == ""

    def test_ascii_passthrough(self):
        text = "Hello, world! 123 foo-bar_baz"
        assert blandify.normalize(text) == text

    def test_default_categories_match_previous_behavior(self):
        assert blandify.normalize("\u2018hi\u2019") == "'hi'"
        assert blandify.normalize("a\u2014b") == "a-b"
        assert blandify.normalize("a\u2010b") == "a-b"
        assert blandify.normalize("a\u200Bb") == "ab"
        assert blandify.normalize("a\u00ADb") == "ab"
        assert blandify.normalize("a\u1680b") == "a b"
        assert blandify.normalize("a\u2000b") == "a b"
        assert blandify.normalize("a\u2001b") == "a b"
        assert blandify.normalize("\u2192") == "->"
        assert blandify.normalize("\u27F6") == "->"
        assert blandify.normalize("\u00BD") == "1/2"
        assert blandify.normalize("\u00D7") == "x"
        assert blandify.normalize("\u2026") == "..."
        assert blandify.normalize("\u2764\uFE0F") == "\u2764"

    def test_umlauts_are_off_by_default(self):
        assert blandify.normalize("ä ö ü ß") == "ä ö ü ß"


class TestNormalizeOptions:
    """Tests for keyword options on normalize()."""

    def test_quotes_toggle(self):
        text = "\u201chello\u201d"
        assert blandify.normalize(text, quotes=False) == text

    def test_dashes_toggle(self):
        text = "a\u2014b"
        assert blandify.normalize(text, dashes=False) == text

    def test_whitespace_toggle(self):
        text = "a\u00A0b"
        assert blandify.normalize(text, whitespace=False) == text

    def test_zero_width_toggle(self):
        text = "a\u200B\u00AD\u2066b"
        assert blandify.normalize(text, zero_width=False) == text

    def test_arrows_toggle(self):
        text = "\u2192"
        assert blandify.normalize(text, arrows=False) == text

    def test_fractions_toggle(self):
        text = "\u00BD"
        assert blandify.normalize(text, fractions=False) == text

    def test_math_toggle(self):
        text = "\u00D7"
        assert blandify.normalize(text, math=False) == text

    def test_symbols_toggle(self):
        text = "\u2026"
        assert blandify.normalize(text, symbols=False) == text

    def test_umlauts_toggle(self):
        assert blandify.normalize("ä ö ü ß", umlauts=True) == "ae oe ue ss"

    def test_control_chars_off_by_default(self):
        assert blandify.normalize("a\u0000b") == "a\u0000b"

    def test_control_chars_toggle(self):
        assert blandify.normalize("a\u0000b\u0001c\u007fd\u0080e", control_chars=True) == "abcde"

    def test_control_chars_reconstructs_corrupted_umlauts(self):
        assert blandify.normalize("\u0000e4", control_chars=True) == "ä"
        assert blandify.normalize("\u0000f6", control_chars=True) == "ö"

    def test_control_chars_and_umlauts_transliterates(self):
        assert blandify.normalize("\u0000e4", control_chars=True, umlauts=True) == "ae"
        assert blandify.normalize("\u0000f6", control_chars=True, umlauts=True) == "oe"

    def test_combined_toggles(self):
        text = "\u201ca\u2014b\u201d"
        assert blandify.normalize(text, quotes=False, dashes=False) == text


class TestCallSemantics:
    """Tests for Python call behavior."""

    def test_boolean_options_are_keyword_only(self):
        with pytest.raises(TypeError):
            blandify.normalize("x", False)

    def test_non_bool_option_type_errors(self):
        with pytest.raises(TypeError):
            blandify.normalize("x", quotes="no")
