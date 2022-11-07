package parser

import (
	"testing"

	"github.com/mrdvince/aap/ast"

	"github.com/mrdvince/aap/lexer"
)

func TestLetStatements(t *testing.T) {
	input := `
let x = 5;
let y = 10;
let foobar = 838383;
`
	l := lexer.New(input)
	p := New(l)
	program := p.ParseProgram()
	if program == nil {
		t.Fatalf("ParseProgram() returned nil")
	}
	if len(program.Statements) != 3 {
		t.Fatalf("program.Statements does not contain 3 statements. got=%d",
			len(program.Statements))
	}
	tests := []struct {
		expectedIdentifier string
	}{
		{"x"},
		{"y"},
		{"foobar"},
	}
	for i, tc := range tests {
		stmt := program.Statements[i]
		if !testLetStatement(t, stmt, tc.expectedIdentifier) {
			return
		}
	}
}

func testLetStatement(t *testing.T, stmt ast.Statement, identifier string /* name e.g x or foo in foo()*/) bool {
	if stmt.TokenLiteral() != "let" {
		t.Errorf("stmt.TokenLiteral not 'let'. got=%q", stmt.TokenLiteral())
		return false
	}
	letStmt, ok := stmt.(*ast.LetStatement) // basically an is instance type assertion
	if !ok {
		t.Errorf("letStmt.Name.Value not '%s'. got=%s", identifier, letStmt.Name.Value)
		return false
	}
	if letStmt.Name.Value != identifier {
		t.Errorf("letStmt.Name.Value not '%s'. got=%s", identifier, letStmt.Name.Value)
		return false
	}
	if letStmt.Name.TokenLiteral() != identifier {
		t.Errorf("stmt.Name not '%s'. got=%s", identifier, letStmt.Name)
		return false
	}
	return true
}
