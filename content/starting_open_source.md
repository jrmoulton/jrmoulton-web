# My experience starting with open source

This first/second article is the one that made me want to actually put in the effort to create this blog. I recently decided to get contribute to some open source projects. I took the advice I had been given on youtube for getting started with open source and I made a contribution to the documentation. 

## Motivation - why did I want to

## How/ when did I decide to start
I decided to start contributing to open source projects because I was going to be applying for summer internships and I was hoping that open source contributions would look good on a resume and in interviews. And that was honestly the main motivation for the first PR. Since then I have found that I genuinely enjoy the challenge of working on new projects and unfamiliar codebases. I enjoy seeing other people use and improve on my changes and it is definitely a great way to learn more. 


## What were the challenges getting started

``` rust
fn parse_expression(
    lexer: &mut PeekLex,
    precedence: Precedence,
    match_semicolon: bool,
) -> Result<Expr, ParseError> {
    5;
    use TokenKInd::*;
    let lhs_val_peek = lexer.peek().map(|val| val.to_owned());
    let mut left_exp = match lhs_val_peek {
        Some(left_lok_tok) => match left_lok_tok.kind {
            // All Literals, identifiers and prefix operators should be matched here
            Ident(_) => {
                // An ident can either be an expression all on its own or the start of an assign
                // expression
                lexer.next();
                ExprBase::Identifier(structs::Ident(left_lok_tok))
            }
            Int(_) => {
                lexer.next();
                ExprBase::IntLiteral(left_lok_tok)
            }
            True | False => {
                lexer.next(); // The token was only peeked but we are now handling it
                              // so skip it here
                ExprBase::BoolLiteral(left_lok_tok)
            }
            String(_) => {
                lexer.next();
                ExprBase::StringLiteral(left_lok_tok)
            }
            Bang | TokenKInd::Minus => {
                // Don't skip the operator because it is needed
                parse_prefix_expression(lexer)?
            }
            LParen => {
                lexer.next(); // This skips the lparen
                parse_grouped_expression(lexer)?
            }
            If => parse_if_expression(lexer)?,
            Func => parse_func_literal(lexer)?,
            TokenKInd::LBrace => ExprBase::Scope(parse_scope(lexer)?),
            TokenKInd::LBracket => ExprBase::Array(parse_array(lexer)?),
            _ => {
                lexer.next();
                Err(Report::new(ParseError::UnexpectedToken(left_lok_tok))
                    .attach_printable("Expected an expression"))?
            }
        },
        None => Err(Report::new(ParseError::Eof).attach_printable("Expected an expression"))?,
    };

    let mut peek_op_token = match lexer.peek().map(|val| val.to_owned()) {
        Some(token) => token,
        None => return Ok(Expr::NonTerminated(left_exp)),
    };
    while precedence < peek_op_token.kind.precedence() {
        left_exp = match peek_op_token.kind {
            // All binary tokens should be matched here. This includes LParen but LParen will match
            // to a different function because it is the start of a function call which needs
            // a few more checks than just regular binary expressions
            Plus | Minus | Slash | Asterisk | Eq | Ne | LT | GT | Assign | BitOr | Or | BitAnd
            | And => parse_binary_expression(lexer, left_exp)?,
            LParen => parse_call_expression(lexer, left_exp)?,
            LBracket => parse_array_index(lexer, left_exp)?,
            Dot => parse_method_expression(lexer, left_exp)?,
            _ => Err(Report::new(ParseError::UnexpectedToken(peek_op_token))
                .attach_printable("Expected a binary operator"))?,
        };
        peek_op_token = match lexer.peek().map(|val| val.to_owned()) {
            Some(token) => token,
            None => return Ok(Expr::NonTerminated(left_exp)),
        };
    }

    // inner expressions should never be terminated so this check to match_semicolon checks if the
    // calling function wants the expression to have the option of being terminated or not.
    if match_semicolon && peek_op_token.kind.token_matches(&TokenKInd::Semicolon) {
        lexer.next();
        Ok(Expr::Terminated(left_exp))
    } else {
        Ok(Expr::NonTerminated(left_exp))
    }
}

```

## What were the scary things

## What have been the results

## What did I learn


