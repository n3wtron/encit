import { Injectable } from '@angular/core';
import { Observable, Subject } from 'rxjs';
import { MatDialog, MatDialogRef } from '@angular/material/dialog';
import { ErrorDialogComponent } from './error-dialog/error-dialog.component';
import { HttpErrorResponse } from '@angular/common/http';

@Injectable({
  providedIn: 'root',
})
export class ErrorService {
  private dialogRef: MatDialogRef<ErrorDialogComponent, any> | undefined;

  constructor(private dialog: MatDialog) {}

  public error(error: object) {
    console.error(error);
    let textError = `${error}`;
    if (error instanceof HttpErrorResponse) {
      textError = `${error.error}`;
    }
    this.dialogRef = this.dialog.open(ErrorDialogComponent, {
      data: { error: textError },
    });
  }

  public resetError() {
    this.dialogRef?.close();
  }
}
